use anyhow::{Context, Result};
use console::{style, Term};
use notify::{Config, Event, EventKind, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::exercise::Exercise;

// Run a single exercise
pub fn run_exercise(exercise: &Exercise, base_path: &Path) -> Result<bool> {
    println!("{}", style(format!("Running exercise: {}", exercise.name)).bold());
    exercise.verify(base_path)
}

// Show hint for an exercise
pub fn show_hint(exercise: &Exercise) -> Result<()> {
    exercise.show_hint();
    Ok(())
}

// List all exercises with their status
pub fn list_exercises(exercises: &[Exercise]) -> Result<()> {
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
    use notify::{Config, RecursiveMode, Watcher};
    use std::sync::mpsc::channel;
    use std::time::Duration;
    use console::style;
    
    // Check if info.toml exists
    if !Path::new("info.toml").exists() {
        return Err(anyhow::anyhow!(
            "Could not find info.toml in the current directory. \
             Please run this command from the project root directory, \
             not from inside the rust-journey-cli directory."
        ));
    }
    
    let exercise = &mut exercises[exercise_index];
    let file_to_watch = base_path.join(&exercise.path);
    let parent_dir = file_to_watch.parent().unwrap_or(Path::new("."));
    
    println!("{}", style(format!("Watching exercise: {}", exercise.name)).bold());
    println!("Press 'q' to quit, 'h' for hint, 'l' for list, 'n' for next exercise");
    
    // Set up the channel for file event notifications
    let (tx, rx) = channel();
    
    // Create an event handler
    struct FileEventHandler {
        tx: std::sync::mpsc::Sender<notify::Event>,
    }
    
    impl notify::EventHandler for FileEventHandler {
        fn handle_event(&mut self, event: std::result::Result<notify::Event, notify::Error>) {
            if let Ok(event) = event {
                let _ = self.tx.send(event);
            }
        }
    }
    
    let event_handler = FileEventHandler { tx };
    
    // Create the watcher with explicit type annotation
    let mut watcher: notify::RecommendedWatcher = notify::Watcher::new(event_handler, Config::default())
        .context("Failed to create file watcher")?;
    
    // Watch the directory
    watcher.watch(parent_dir, RecursiveMode::Recursive)
        .context("Failed to watch directory")?;
    
    let term = Term::stdout();
    
    loop {
        // Check for file changes
        match rx.try_recv() {
            Ok(event) if matches!(event.kind, notify::EventKind::Modify(_)) => {
                let exercise = &mut exercises[exercise_index];
                if exercise.verify(base_path)? {
                    exercise.completed = true;
                    save_exercise_status(exercises, status_path)?;
                    
                    if let Some(next_idx) = find_next_exercise(exercises) {
                        println!("{}", style("Exercise completed! Move to next? [y/n]").green().bold());
                        if let Ok(key) = term.read_char() {
                            if key == 'y' {
                                return watch_exercise(next_idx, exercises, base_path, status_path);
                            }
                        }
                    } else {
                        println!("{}", style("All exercises completed! Congratulations!").green().bold());
                        return Ok(());
                    }
                }
            },
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                // No file change event, this is normal
            },
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                return Err(anyhow::anyhow!("File watcher disconnected unexpectedly"));
            },
            _ => {}
        }
        
        // Check for keyboard input
        if let Ok(key) = term.read_char() {
            match key {
                'q' => {
                    println!("Exiting watch mode");
                    break;
                },
                'h' => {
                    exercises[exercise_index].show_hint();
                },
                'l' => {
                    list_exercises(exercises)?;
                },
                'n' => {
                    if let Some(next_idx) = find_next_exercise(exercises) {
                        return watch_exercise(next_idx, exercises, base_path, status_path);
                    } else {
                        println!("No more exercises to complete!");
                    }
                },
                _ => {}
            }
        }
        
        std::thread::sleep(Duration::from_millis(100));
    }
    
    Ok(())
} 