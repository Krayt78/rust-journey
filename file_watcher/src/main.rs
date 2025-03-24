// file_watcher/src/main.rs
mod commands;
mod exercise;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use console::style;
use notify::{Config, EventKind, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{fs, process};
use chrono::Local;

use crate::commands::{clear_console, find_next_exercise, list_exercises, load_exercise_status, run_exercise, save_exercise_status, show_hint, watch_exercise};
use crate::exercise::load_exercises;

#[derive(Parser)]
#[clap(author, version, about = "A CLI tool for watching files and running Rust Journey exercises")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
    
    /// Path to the file to watch (for file watcher mode)
    #[clap(short, long)]
    file: Option<String>,

    /// Optional command to execute when the file changes (for file watcher mode)
    #[clap(short, long)]
    command_to_run: Option<String>,

    /// Time in milliseconds to debounce file change events (for file watcher mode)
    #[clap(short, long, default_value = "500")]
    debounce: u64,
    
    /// Enable verbose debug output (for file watcher mode)
    #[clap(short, long)]
    verbose: bool,
    
    #[clap(long, default_value = ".")]
    base_path: PathBuf,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Run a specific exercise
    Run {
        /// Name of the exercise to run
        name: Option<String>,
    },
    
    /// Run the next incomplete exercise
    Next,
    
    /// List all exercises and their completion status
    List,
    
    /// Show a hint for a specific exercise
    Hint {
        /// Name of the exercise to get a hint for
        name: Option<String>,
    },
    
    /// Watch for file changes and verify exercises
    Watch,
    
    /// Verify all exercises
    Verify,
    
    /// Reset exercise progress by deleting the status file
    Reset,
    
    /// Watch a specific file (original file_watcher functionality)
    WatchFile,
}

// The original file_watcher logic, moved to a function
fn handle_file_change(file_path: &Path, command: &Option<String>) {
    // Read file content
    match fs::read_to_string(file_path) {
        Ok(content) => {
            println!("File size: {} bytes", content.len());
            
            // Example custom logic: Count lines, words, and characters
            let line_count = content.lines().count();
            let word_count = content.split_whitespace().count();
            let char_count = content.chars().count();
            
            println!("Stats: {} lines, {} words, {} characters", 
                     line_count, word_count, char_count);
            
            // If this is a Rust file, try to run the tests
            if file_path.extension().map_or(false, |ext| ext == "rs") {
                println!("Running tests for Rust file...");
                
                // For example files, run the tests directly on the file
                let parent_dir = file_path.parent().unwrap_or(Path::new("."));
                let file_name = file_path.file_name().unwrap_or_default().to_string_lossy();
                
                // Generate a unique output file name
                let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
                let output_name = format!("test_output_{}", timestamp);
                
                let test_command = format!("cd {} && rustc --edition=2021 --test {} -o {} && ./{}",
                                       parent_dir.display(),
                                       file_name,
                                       output_name,
                                       output_name);
                                      
                println!("Executing: {}", test_command);
                
                #[cfg(target_os = "windows")]
                let command_process = process::Command::new("cmd")
                    .args(["/C", &test_command])
                    .spawn();
                
                #[cfg(not(target_os = "windows"))]
                let command_process = process::Command::new("sh")
                    .args(["-c", &test_command])
                    .spawn();
                
                match command_process {
                    Ok(mut child) => {
                        match child.wait() {
                            Ok(status) => println!("Tests exited with status: {}", status),
                            Err(e) => eprintln!("Error waiting for tests: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Failed to run tests: {}", e),
                }
                
                // Clean up the test executable
                let _ = fs::remove_file(parent_dir.join(output_name));
            }
            
            // Execute command if provided
            if let Some(cmd) = command {
                println!("Executing command: {}", cmd);
                
                #[cfg(target_os = "windows")]
                let command_process = process::Command::new("cmd")
                    .args(["/C", cmd])
                    .spawn();
                
                #[cfg(not(target_os = "windows"))]
                let command_process = process::Command::new("sh")
                    .args(["-c", cmd])
                    .spawn();
                
                match command_process {
                    Ok(mut child) => {
                        match child.wait() {
                            Ok(status) => println!("Command exited with status: {}", status),
                            Err(e) => eprintln!("Error waiting for command: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Failed to execute command: {}", e),
                }
            }
            
            println!("-------------------------------------------");
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

// The original file_watcher logic, moved to a function
fn watch_file(args: &Cli) -> Result<()> {
    let file_path = PathBuf::from(args.file.as_ref().unwrap()).canonicalize()
        .context("Error resolving path")?;

    if !file_path.exists() {
        return Err(anyhow::anyhow!("Error: The file {} does not exist", args.file.as_ref().unwrap()));
    }

    println!("Starting file watcher for: {}", file_path.display());
    println!("Canonical path: {}", file_path.display());
    println!("Parent directory: {:?}", file_path.parent());
    
    // Set up watcher for the parent directory instead of the file directly
    let (tx, rx) = channel();
    
    let config = Config::default()
        .with_poll_interval(Duration::from_millis(args.debounce));
    
    // Create an event handler
    struct FileEventHandler {
        tx: std::sync::mpsc::Sender<notify::Event>,
        target_file: PathBuf,
        verbose: bool,
    }
    
    impl notify::EventHandler for FileEventHandler {
        fn handle_event(&mut self, event: std::result::Result<notify::Event, notify::Error>) {
            if let Ok(event) = event {
                if self.verbose {
                    println!("Event received: {:?}", event);
                    println!("Event kind: {:?}", event.kind);
                    println!("Event paths: {:?}", event.paths);
                }
                
                // Check if any of the paths in the event match our target file
                for path in &event.paths {
                    if self.verbose {
                        println!("Comparing {} with {}", path.display(), self.target_file.display());
                    }
                    
                    if path == &self.target_file {
                        if self.verbose {
                            println!("Path match found!");
                        }
                        
                        // Only process certain event types: create, modify, or remove
                        match event.kind {
                            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                                // Send the event
                                let _ = self.tx.send(event.clone());
                                return;
                            },
                            _ => {
                                if self.verbose {
                                    println!("Ignoring event type: {:?}", event.kind);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    let event_handler = FileEventHandler { 
        tx, 
        target_file: file_path.clone(),
        verbose: args.verbose,
    };
    
    let mut watcher: notify::RecommendedWatcher = notify::Watcher::new(event_handler, config)
        .context("Failed to create file watcher")?;
    
    // Watch the parent directory instead of just the file
    let watch_path = file_path.parent().unwrap_or(Path::new("."));
    println!("Watching directory: {}", watch_path.display());
    
    watcher.watch(watch_path, RecursiveMode::NonRecursive)
        .context("Failed to watch directory")?;

    println!("Watching for changes. Press Ctrl+C to stop.");
    println!("Watched file name: {}", file_path.file_name().unwrap_or_default().to_string_lossy());
    
    // Process events
    loop {
        match rx.recv() {
            Ok(_event) => {
                println!("[{}] File changed: {}", 
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    file_path.display());
                
                // Execute custom logic
                handle_file_change(&file_path, &args.command_to_run);
            }
            Err(e) => {
                eprintln!("Error receiving event: {:?}", e);
                break;
            }
        }
    }
    
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let base_path = Path::new(&cli.base_path).canonicalize()
        .context("Failed to canonicalize base path")?;
    
    // If file is provided but no command is provided, use the WatchFile command
    let command = if cli.file.is_some() && cli.command.is_none() {
        Some(Commands::WatchFile)
    } else {
        cli.command.clone() // Clone here to prevent partial move
    };
    
    match command {
        Some(cmd) => {
            match cmd {
                Commands::Run { name } => {
                    // Path to the info.toml file
                    let info_path = Path::new("info.toml");
                    
                    // Check if info.toml exists before proceeding
                    if !info_path.exists() {
                        clear_console();
                        eprintln!("{}", style("ERROR: Could not find info.toml in the current directory.").red().bold());
                        eprintln!("{}", style("Please run this command from the project root directory.").yellow());
                        return Err(anyhow::anyhow!("info.toml not found"));
                    }
                    
                    // Path to the status file
                    let status_path = Path::new(".rust-journey-status");
                    
                    // Load exercises
                    let mut exercises = load_exercises(info_path)
                        .context("Failed to load exercises")?;
                    
                    // Load exercise status
                    load_exercise_status(&mut exercises, status_path)
                        .context("Failed to load exercise status")?;
                    
                    let exercise_index = match name {
                        Some(name) => {
                            exercises.iter().position(|e| e.name == name)
                                .context("Exercise not found")?
                        },
                        None => {
                            find_next_exercise(&exercises)
                                .context("No incomplete exercises found")?
                        }
                    };
                    
                    let exercise = &mut exercises[exercise_index];
                    let completed = run_exercise(exercise, &base_path)?;
                    
                    if completed {
                        exercise.completed = true;
                        save_exercise_status(&exercises, status_path)?;
                        println!("Exercise completed!");
                    }
                },
                
                Commands::Next => {
                    // Path to the info.toml file
                    let info_path = Path::new("info.toml");
                    
                    // Check if info.toml exists before proceeding
                    if !info_path.exists() {
                        clear_console();
                        eprintln!("{}", style("ERROR: Could not find info.toml in the current directory.").red().bold());
                        eprintln!("{}", style("Please run this command from the project root directory.").yellow());
                        return Err(anyhow::anyhow!("info.toml not found"));
                    }
                    
                    // Path to the status file
                    let status_path = Path::new(".rust-journey-status");
                    
                    // Load exercises
                    let mut exercises = load_exercises(info_path)
                        .context("Failed to load exercises")?;
                    
                    // Load exercise status
                    load_exercise_status(&mut exercises, status_path)
                        .context("Failed to load exercise status")?;
                    
                    let exercise_index = find_next_exercise(&exercises);
                    
                    if let Some(index) = exercise_index {
                        let exercise = &mut exercises[index];
                        let completed = run_exercise(exercise, &base_path)?;
                        
                        if completed {
                            exercise.completed = true;
                            save_exercise_status(&exercises, status_path)?;
                            println!("{}", style("Exercise completed!").green().bold());
                        }
                    } else {
                        clear_console();
                        println!("{}", style("🎉 All exercises completed! Congratulations! 🎉").green().bold());
                        println!("\nYou've completed all exercises in the Rust Journey course!");
                        println!("\nIf you want to reset your progress and start over, you can delete the .rust-journey-status file.");
                    }
                },
                
                Commands::List => {
                    // Path to the info.toml file
                    let info_path = Path::new("info.toml");
                    
                    // Check if info.toml exists before proceeding
                    if !info_path.exists() {
                        clear_console();
                        eprintln!("{}", style("ERROR: Could not find info.toml in the current directory.").red().bold());
                        eprintln!("{}", style("Please run this command from the project root directory.").yellow());
                        return Err(anyhow::anyhow!("info.toml not found"));
                    }
                    
                    // Load exercises
                    let mut exercises = load_exercises(info_path)
                        .context("Failed to load exercises")?;
                    
                    // Load exercise status
                    load_exercise_status(&mut exercises, Path::new(".rust-journey-status"))
                        .context("Failed to load exercise status")?;
                    
                    list_exercises(&exercises)?;
                },
                
                Commands::Hint { name } => {
                    // Path to the info.toml file
                    let info_path = Path::new("info.toml");
                    
                    // Check if info.toml exists before proceeding
                    if !info_path.exists() {
                        clear_console();
                        eprintln!("{}", style("ERROR: Could not find info.toml in the current directory.").red().bold());
                        eprintln!("{}", style("Please run this command from the project root directory.").yellow());
                        return Err(anyhow::anyhow!("info.toml not found"));
                    }
                    
                    // Load exercises
                    let mut exercises = load_exercises(info_path)
                        .context("Failed to load exercises")?;
                    
                    // Load exercise status
                    load_exercise_status(&mut exercises, Path::new(".rust-journey-status"))
                        .context("Failed to load exercise status")?;
                    
                    let exercise_index = match name {
                        Some(name) => {
                            exercises.iter().position(|e| e.name == name)
                                .context("Exercise not found")?
                        },
                        None => {
                            find_next_exercise(&exercises)
                                .context("No incomplete exercises found")?
                        }
                    };
                    
                    show_hint(&exercises[exercise_index])?;
                },
                
                Commands::Watch => {
                    // Path to the info.toml file
                    let info_path = Path::new("info.toml");
                    
                    // Check if info.toml exists before proceeding
                    if !info_path.exists() {
                        clear_console();
                        eprintln!("{}", style("ERROR: Could not find info.toml in the current directory.").red().bold());
                        eprintln!("{}", style("Please run this command from the project root directory.").yellow());
                        return Err(anyhow::anyhow!("info.toml not found"));
                    }
                    
                    // Path to the status file
                    let status_path = Path::new(".rust-journey-status");
                    
                    // Load exercises
                    let mut exercises = load_exercises(info_path)
                        .context("Failed to load exercises")?;
                    
                    // Load exercise status
                    load_exercise_status(&mut exercises, status_path)
                        .context("Failed to load exercise status")?;
                    
                    let exercise_index = find_next_exercise(&exercises);
                    
                    if let Some(index) = exercise_index {
                        watch_exercise(index, &mut exercises, &base_path, status_path)?;
                    } else {
                        clear_console();
                        println!("{}", style("🎉 All exercises completed! Congratulations! 🎉").green().bold());
                        println!("\nYou've completed all exercises in the Rust Journey course!");
                        println!("\nIf you want to reset your progress and start over, you can delete the .rust-journey-status file.");
                    }
                },
                
                Commands::Verify => {
                    // Path to the info.toml file
                    let info_path = Path::new("info.toml");
                    
                    // Check if info.toml exists before proceeding
                    if !info_path.exists() {
                        clear_console();
                        eprintln!("{}", style("ERROR: Could not find info.toml in the current directory.").red().bold());
                        eprintln!("{}", style("Please run this command from the project root directory.").yellow());
                        return Err(anyhow::anyhow!("info.toml not found"));
                    }
                    
                    // Path to the status file
                    let status_path = Path::new(".rust-journey-status");
                    
                    // Load exercises
                    let mut exercises = load_exercises(info_path)
                        .context("Failed to load exercises")?;
                    
                    // Load exercise status
                    load_exercise_status(&mut exercises, status_path)
                        .context("Failed to load exercise status")?;
                    
                    clear_console();
                    println!("{}", style("Verifying all exercises...").cyan().bold());
                    let mut all_passing = true;
                    let total_exercises = exercises.len();
                    
                    for i in 0..total_exercises {
                        let exercise = &mut exercises[i];
                        println!("\n{}", style(format!("Exercise {}/{}: {}", i + 1, total_exercises, exercise.name)).blue());
                        let passed = run_exercise(exercise, &base_path)?;
                        
                        if passed {
                            exercise.completed = true;
                        } else {
                            all_passing = false;
                        }
                    }
                    
                    save_exercise_status(&exercises, status_path)?;
                    
                    if all_passing {
                        println!("\n{}", style("🎉 All exercises pass! Congratulations! 🎉").green().bold());
                    } else {
                        println!("\n{}", style("Some exercises failed. Keep working on them!").yellow().bold());
                        println!("Use 'file_watcher watch' to focus on the next incomplete exercise.");
                    }
                },
                
                Commands::Reset => {
                    clear_console();
                    let status_path = Path::new(".rust-journey-status");
                    if status_path.exists() {
                        match std::fs::remove_file(status_path) {
                            Ok(_) => {
                                println!("{}", style("✅ Progress reset successfully!").green().bold());
                                println!("All exercises are now marked as incomplete.");
                                println!("\nRun 'file_watcher list' to see all exercises.");
                            },
                            Err(e) => {
                                println!("{}", style("❌ Failed to reset progress:").red().bold());
                                println!("{}", e);
                                return Err(anyhow::anyhow!("Failed to delete status file: {}", e));
                            }
                        }
                    } else {
                        println!("{}", style("ℹ️ No progress file found.").blue().bold());
                        println!("All exercises are already marked as incomplete.");
                    }
                },
                
                Commands::WatchFile => {
                    // Make sure file is set
                    if cli.file.is_none() {
                        return Err(anyhow::anyhow!("Error: --file parameter is required for watch-file mode"));
                    }
                    
                    watch_file(&cli)?;
                },
            }
        },
        None => {
            // Print usage if no command or file is provided
            println!("Please provide a command or use --file option to watch a file.");
            println!("Run with --help for more information.");
        }
    }
    
    Ok(())
}