use anyhow::{Context, Result};
use console::{style, Term};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::stdin;
use termion::input::TermRead;

use crate::exercise::Exercise;

// Function to clear the console (cross-platform)
pub fn clear_console() {
    if cfg!(target_os = "windows") {
        // For Windows
        let _ = Command::new("cmd")
            .args(["/c", "cls"])
            .status();
    } else {
        // For Unix-like systems (Linux, macOS)
        let _ = Command::new("clear")
            .status();
    }
}

// Run a single exercise
pub fn run_exercise(exercise: &Exercise, base_path: &Path) -> Result<bool> {
    clear_console();
    println!("{}", style(format!("Running exercise: {}", exercise.name)).bold());
    exercise.verify(base_path)
}

// Show hint for an exercise
pub fn show_hint(exercise: &Exercise) -> Result<()> {
    clear_console();
    exercise.show_hint();
    Ok(())
}

// List all exercises with their status
pub fn list_exercises(exercises: &[Exercise]) -> Result<()> {
    clear_console();
    println!("{}", style("Rust Journey Exercises:").bold().underlined());
    println!();
    
    for (i, exercise) in exercises.iter().enumerate() {
        let status = if exercise.completed {
            style("✓").green().to_string()
        } else {
            style("✗").red().to_string()
        };
        
        println!("{:3}. [{}] {}", i + 1, status, exercise.name);
    }
    
    Ok(())
}

// Find the next incomplete exercise
pub fn find_next_exercise(exercises: &[Exercise]) -> Option<usize> {
    exercises.iter().position(|e| !e.completed)
}

// Save the exercise status to a file
pub fn save_exercise_status(exercises: &[Exercise], path: &Path) -> Result<()> {
    let mut content = String::new();
    
    for exercise in exercises {
        content.push_str(&format!("{} = {}\n", exercise.name, exercise.completed));
    }
    
    std::fs::write(path, content)
        .context(format!("Failed to write status file at {}", path.display()))?;
    
    Ok(())
}

// Load the exercise status from a file
pub fn load_exercise_status(exercises: &mut [Exercise], path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    
    let content = std::fs::read_to_string(path)
        .context(format!("Failed to read status file at {}", path.display()))?;
    
    for line in content.lines() {
        if let Some((name, status)) = line.split_once(" = ") {
            if let Ok(completed) = status.parse::<bool>() {
                if let Some(exercise) = exercises.iter_mut().find(|e| e.name == name) {
                    exercise.completed = completed;
                }
            }
        }
    }
    
    Ok(())
}

// Watch for changes to exercise files
pub fn watch_exercise(exercise_index: usize, exercises: &mut Vec<Exercise>, base_path: &Path, status_path: &Path) -> Result<()> {
    use notify::{Config, RecursiveMode, Watcher, EventKind};
    use std::sync::mpsc::{channel, TryRecvError};
    use std::time::Duration;
    use console::style;
    
    // Check if info.toml exists
    if !Path::new("info.toml").exists() {
        return Err(anyhow::anyhow!(
            "Could not find info.toml in the current directory. \
             Please run this command from the project root directory, \
             not from inside the file_watcher directory."
        ));
    }
    
    // Get exercise name first before borrowing mutably
    let exercise_name = exercises[exercise_index].name.clone();
    let file_to_watch = base_path.join(&exercises[exercise_index].path);
    let file_name = file_to_watch.file_name().unwrap_or_default().to_string_lossy().to_string();
    let parent_dir = file_to_watch.parent().unwrap_or(Path::new("."));
    let file_path = PathBuf::from(file_name.clone());
    
    clear_console();
    println!("{}", style(format!("Watching exercise: {}", exercise_name)).bold());
    println!("Press 'q' to quit, 'h' for hint, 'l' for list, 'n' for next exercise");
    
    // Immediately verify the exercise upon entering watch mode
    println!("\n{}", style("Initial verification:").cyan().bold());
    // Need to scope the mutable borrow
    let completed = {
        let exercise = &mut exercises[exercise_index];
        let result = exercise.verify(base_path)?;
        if result {
            exercise.completed = true;
        }
        result
    };
    
    // Check if it's already completed on first try
    if completed {
        save_exercise_status(exercises, status_path)?;
        
        if let Some(next_idx) = find_next_exercise(exercises) {
            println!("{}", style("Exercise completed! Move to next? [y/n]").green().bold());
            
            // Wait for y/n input
            let stdin = stdin();
            for key in stdin.keys() {
                if let Ok(key) = key {
                    match key {
                        termion::event::Key::Char('y') => {
                            return watch_exercise(next_idx, exercises, base_path, status_path);
                        },
                        _ => break,
                    }
                }
                break;
            }
        } else {
            println!("{}", style("All exercises completed! Congratulations!").green().bold());
            println!("Press any key to exit...");
            
            let stdin = stdin();
            let _ = stdin.keys().next(); // Wait for any key
            return Ok(());
        }
    }
    
    println!("\n{}", style("Watching for changes...").dim());
    println!("{}", style(format!("Target file: {}", file_name)).dim());
    
    // Set up the channel for file event notifications with a longer timeout
    let (tx, rx) = channel();
    
    // Create an event handler
    struct FileEventHandler {
        tx: std::sync::mpsc::Sender<notify::Event>,
        target_file: PathBuf,
    }
    
    impl notify::EventHandler for FileEventHandler {
        fn handle_event(&mut self, event: std::result::Result<notify::Event, notify::Error>) {
            if let Ok(event) = event {
                // Check if any of the paths in the event match our target file
                for path in &event.paths {
                    if path == &self.target_file {
                        // Only process certain event types: create, modify, or remove
                        match event.kind {
                            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                                // Send the event
                                let _ = self.tx.send(event.clone());
                                return;
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    
    let event_handler = FileEventHandler { 
        tx, 
        target_file: file_path.clone(),
    };
    
    // Create the watcher with explicit type annotation and debounced config
    let config = Config::default()
        .with_poll_interval(Duration::from_millis(100));
    
    let mut watcher: notify::RecommendedWatcher = notify::Watcher::new(event_handler, config)
        .context("Failed to create file watcher")?;
    
    // Watch the directory recursively to catch all events
    watcher.watch(parent_dir, RecursiveMode::Recursive)
        .context("Failed to watch directory")?;
    
    // Set up non-blocking input
    let stdin = stdin();
    let mut keys = stdin.keys();
    
    let term = Term::stdout();
    let mut last_verify_time = std::time::Instant::now();
    
    loop {
        // Check for file changes with a very short timeout to be responsive
        match rx.try_recv() {
            Ok(event) => {
                // Only react if we haven't verified in the last 100ms (debounce)
                let now = std::time::Instant::now();
                if now.duration_since(last_verify_time) > Duration::from_millis(100) {
                    last_verify_time = now;
                    
                    clear_console();
                    println!("{}", style("File changed! Verifying...").cyan().bold());
                    
                    // Need to scope the mutable borrow
                    let completed = {
                        let exercise = &mut exercises[exercise_index];
                        let result = exercise.verify(base_path)?;
                        if result {
                            exercise.completed = true;
                        }
                        result
                    };
                    
                    // Only use immutable references after the mutable borrow is done
                    if completed {
                        save_exercise_status(exercises, status_path)?;
                        
                        if let Some(next_idx) = find_next_exercise(exercises) {
                            println!("{}", style("Exercise completed! Move to next? [y/n]").green().bold());
                            
                            // Non-blocking input check for moving to next exercise
                            if let Some(Ok(key)) = keys.next() {
                                match key {
                                    termion::event::Key::Char('y') => {
                                        return watch_exercise(next_idx, exercises, base_path, status_path);
                                    },
                                    _ => {}
                                }
                            }
                        } else {
                            println!("{}", style("All exercises completed! Congratulations!").green().bold());
                            break;
                        }
                    }
                    
                    println!("\n{}", style("Watching for changes...").dim());
                    println!("Press 'q' to quit, 'h' for hint, 'l' for list, 'n' for next exercise");
                }
            },
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => break,
        }
        
        // Check for keyboard input
        if let Some(Ok(key)) = keys.next() {
            match key {
                termion::event::Key::Char('q') => {
                    clear_console();
                    println!("Exiting watch mode.");
                    break;
                },
                termion::event::Key::Char('h') => {
                    clear_console();
                    println!("{}", style(format!("Hint for {}:", exercise_name)).yellow().bold());
                    exercises[exercise_index].show_hint();
                    println!("\nPress any key to continue watching...");
                    term.read_char()?;
                    clear_console();
                    println!("{}", style(format!("Watching exercise: {}", exercise_name)).bold());
                    println!("Press 'q' to quit, 'h' for hint, 'l' for list, 'n' for next exercise");
                },
                termion::event::Key::Char('l') => {
                    list_exercises(exercises)?;
                    println!("\nPress any key to continue watching...");
                    term.read_char()?;
                    clear_console();
                    println!("{}", style(format!("Watching exercise: {}", exercise_name)).bold());
                    println!("Press 'q' to quit, 'h' for hint, 'l' for list, 'n' for next exercise");
                },
                termion::event::Key::Char('n') => {
                    if let Some(next_idx) = find_next_exercise(exercises) {
                        return watch_exercise(next_idx, exercises, base_path, status_path);
                    } else {
                        clear_console();
                        println!("{}", style("All exercises completed! Congratulations!").green().bold());
                        break;
                    }
                },
                _ => {},
            }
        }
        
        // Sleep briefly to avoid burning CPU
        std::thread::sleep(Duration::from_millis(10));
    }
    
    Ok(())
} 