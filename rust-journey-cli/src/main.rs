mod commands;
mod exercise;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use console::style;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::commands::{find_next_exercise, list_exercises, load_exercise_status, run_exercise, save_exercise_status, show_hint, watch_exercise};
use crate::exercise::load_exercises;

#[derive(Parser)]
#[clap(author, version, about = "A CLI tool for the Rust Journey course")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    
    #[clap(long, default_value = ".")]
    base_path: PathBuf,
}

#[derive(Subcommand)]
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
}

// Function to clear the console (cross-platform)
fn clear_console() {
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

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let base_path = Path::new(&cli.base_path).canonicalize()
        .context("Failed to canonicalize base path")?;
    
    // Path to the info.toml file
    let info_path = Path::new("info.toml");
    
    // Check if info.toml exists before proceeding
    if !info_path.exists() {
        clear_console();
        eprintln!("{}", style("ERROR: Could not find info.toml in the current directory.").red().bold());
        eprintln!("{}", style("Please run this command from the project root directory, not from inside the rust-journey-cli directory.").yellow());
        eprintln!("\nIf you're in the rust-journey-cli directory, try: cd .. && rust-journey [command]");
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
    
    match cli.command {
        Commands::Run { name } => {
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
            list_exercises(&exercises)?;
        },
        
        Commands::Hint { name } => {
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
                println!("Use 'rust-journey watch' to focus on the next incomplete exercise.");
            }
        },
        
        Commands::Reset => {
            clear_console();
            if status_path.exists() {
                match std::fs::remove_file(status_path) {
                    Ok(_) => {
                        println!("{}", style("✅ Progress reset successfully!").green().bold());
                        println!("All exercises are now marked as incomplete.");
                        println!("\nRun 'rust-journey list' to see all exercises.");
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
    }
    
    Ok(())
}
