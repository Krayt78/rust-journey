use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::Command;

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

impl Exercise {
    // Verify if the exercise compiles or passes tests
    pub fn verify(&self, base_path: &Path) -> Result<bool> {
        let full_path = base_path.join(&self.path);
        
        match self.mode {
            Mode::Compile => {
                println!("Compiling {}...", self.name);
                let output = Command::new("rustc")
                    .arg("--edition=2021")
                    .arg(&full_path)
                    .output()
                    .context(format!("Failed to execute rustc on {}", full_path.display()))?;
                
                if output.status.success() {
                    println!("Successfully compiled {}", self.name);
                    Ok(true)
                } else {
                    println!("Failed to compile {}:", self.name);
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                    Ok(false)
                }
            },
            Mode::Test => {
                println!("Testing {}...", self.name);
                let output = Command::new("rustc")
                    .arg("--edition=2021")
                    .arg("--test")
                    .arg(&full_path)
                    .output()
                    .context(format!("Failed to compile test for {}", full_path.display()))?;
                
                if !output.status.success() {
                    println!("Failed to compile test for {}:", self.name);
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                    return Ok(false);
                }
                
                // If compilation successful, run the test
                let test_executable = full_path.with_extension("");
                let run_output = Command::new(&test_executable)
                    .output()
                    .context(format!("Failed to run test for {}", full_path.display()))?;
                
                // Clean up the test executable
                std::fs::remove_file(test_executable).ok();
                
                if run_output.status.success() {
                    println!("Tests passed for {}", self.name);
                    Ok(true)
                } else {
                    println!("Tests failed for {}:", self.name);
                    println!("{}", String::from_utf8_lossy(&run_output.stdout));
                    Ok(false)
                }
            }
        }
    }
    
    // Show hint for the exercise
    pub fn show_hint(&self) {
        println!("Hint for {}:", self.name);
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