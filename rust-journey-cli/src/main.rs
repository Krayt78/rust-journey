mod commands;
mod exercise;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

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
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let base_path = Path::new(&cli.base_path).canonicalize()
        .context("Failed to canonicalize base path")?;
    
    // Path to the info.toml file
    let info_path = Path::new("info.toml");
    
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
            let exercise_index = find_next_exercise(&exercises)
                .context("No incomplete exercises found")?;
            
            let exercise = &mut exercises[exercise_index];
            let completed = run_exercise(exercise, &base_path)?;
            
            if completed {
                exercise.completed = true;
                save_exercise_status(&exercises, status_path)?;
                println!("Exercise completed!");
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
            let exercise_index = find_next_exercise(&exercises)
                .context("No incomplete exercises found")?;
            
            watch_exercise(exercise_index, &mut exercises, &base_path, status_path)?;
        },
        
        Commands::Verify => {
            println!("Verifying all exercises...");
            let mut all_passing = true;
            let total_exercises = exercises.len();
            
            for i in 0..total_exercises {
                let exercise = &mut exercises[i];
                println!("\nExercise {}/{}: {}", i + 1, total_exercises, exercise.name);
                let passed = run_exercise(exercise, &base_path)?;
                
                if passed {
                    exercise.completed = true;
                } else {
                    all_passing = false;
                }
            }
            
            save_exercise_status(&exercises, status_path)?;
            
            if all_passing {
                println!("\nAll exercises pass! Congratulations!");
            } else {
                println!("\nSome exercises failed. Keep working on them!");
            }
        },
    }
    
    Ok(())
}
