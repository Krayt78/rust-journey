use std::fs;
use std::process;
use anyhow::{Context, Result};
use console::style;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::Command;
use chrono::Local;
#[derive(Debug, Deserialize, Clone)]
pub struct Exercise {
    pub name: String,
    pub path: PathBuf,
    pub mode: Mode,
    pub hint: String,
    #[serde(default)]
    pub completed: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Compile,
    Test,
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

impl Exercise {
    // Verify if the exercise compiles or passes tests
    pub fn verify(&self, base_path: &Path) -> Result<bool> {
        let full_path = base_path.join(&self.path);

        println!("Initial verification: {} Full path: {:?}", self.name, full_path);
        
        match self.mode {
            Mode::Compile => {
                println!("{}", style(format!("Compiling {}...", self.name)).cyan().bold());
                let output = Command::new("rustc")
                    .arg("--edition=2021")
                    .arg(&full_path)
                    .output()
                    .context(format!("Failed to execute rustc on {}", full_path.display()))?;
                
                if output.status.success() {
                    println!("{}", style(format!("✅ Successfully compiled {}", self.name)).green().bold());
                    Ok(true)
                } else {
                    println!("{}", style(format!("❌ Failed to compile {}:", self.name)).red().bold());
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                    Ok(false)
                }
            },
            Mode::Test => {
                println!("{}", style(format!("Testing {}...", self.name)).cyan().bold());
                // For example files, run the tests directly on the file
                let parent_dir = full_path.parent().unwrap_or(Path::new("."));
                let file_name = full_path.file_name().unwrap_or_default().to_string_lossy();
                
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

                let mut success = true;
                
                match command_process {
                    Ok(mut child) => {
                        match child.wait() {
                            Ok(status) => {
                                if status.success() {
                                    println!("{}", style(format!("✅ Tests passed for {}", self.name)).green().bold());
                                } else {
                                    println!("{}", style(format!("❌ Tests failed for {}", self.name)).red().bold());
                                    success = false;
                                }
                            },
                            Err(e) => {
                                println!("{}", style(format!("❌ Tests failed for {}", self.name)).red().bold());
                                success = false;
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed to run tests: {}", e),
                }
                
                // Clean up the test executable
                let _ = fs::remove_file(parent_dir.join(output_name));

                Ok(success)
            }
        }
    }
    
    // Show hint for the exercise
    pub fn show_hint(&self) {
        println!("{}", style(format!("Hint for {}:", self.name)).yellow().bold());
        println!("{}", self.hint);
    }
}

// Load exercises from TOML file
pub fn load_exercises(path: &Path) -> Result<Vec<Exercise>> {
    let content = std::fs::read_to_string(path)
        .context(format!("Failed to read exercises file at {}", path.display()))?;
    
    // Direct approach using from_str for the whole file
    let config: toml::Table = toml::from_str(&content)
        .context("Failed to parse TOML content")?;
    
    let exercises = config.get("exercises")
        .context("No 'exercises' field in TOML file")?;
    
    // Convert to Vec<Exercise> directly
    let exercises: Vec<Exercise> = match exercises {
        toml::Value::Array(arr) => {
            arr.iter()
                .map(|val| toml::from_str(&toml::to_string(val).unwrap()))
                .collect::<std::result::Result<Vec<Exercise>, _>>()
                .context("Failed to parse exercises from TOML")?
        },
        _ => return Err(anyhow::anyhow!("'exercises' is not an array")),
    };
    
    Ok(exercises)
} 