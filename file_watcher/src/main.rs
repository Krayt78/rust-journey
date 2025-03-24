// file_watcher/src/main.rs
use clap::Parser;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{fs, process};
use chrono::Local;

#[derive(Parser, Debug)]
#[clap(author, version, about = "A CLI tool that watches a file and executes logic when it changes")]
struct Args {
    /// Path to the file to watch
    #[clap(short, long)]
    file: String,

    /// Optional command to execute when the file changes
    #[clap(short, long)]
    command: Option<String>,

    /// Time in milliseconds to debounce file change events
    #[clap(short, long, default_value = "500")]
    debounce: u64,
    
    /// Enable verbose debug output
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let file_path = PathBuf::from(&args.file).canonicalize().unwrap_or_else(|e| {
        eprintln!("Error resolving path: {}", e);
        process::exit(1);
    });

    if !file_path.exists() {
        eprintln!("Error: The file {} does not exist", args.file);
        process::exit(1);
    }

    println!("Starting file watcher for: {}", file_path.display());
    println!("Canonical path: {}", file_path.display());
    println!("Parent directory: {:?}", file_path.parent());
    
    // Set up watcher for the parent directory instead of the file directly
    let (tx, rx) = channel();
    
    let mut watcher = RecommendedWatcher::new(
        tx,
        Config::default().with_poll_interval(Duration::from_millis(args.debounce))
    ).unwrap();

    // Watch the parent directory instead of just the file
    let watch_path = file_path.parent().unwrap_or(Path::new("."));
    println!("Watching directory: {}", watch_path.display());
    
    watcher.watch(watch_path, RecursiveMode::NonRecursive).unwrap();

    println!("Watching for changes. Press Ctrl+C to stop.");
    println!("Watched file name: {}", file_path.file_name().unwrap_or_default().to_string_lossy());
    
    // Process events
    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if args.verbose {
                    println!("Event received: {:?}", event);
                    println!("Event kind: {:?}", event.kind);
                    println!("Event paths: {:?}", event.paths);
                }
                
                // Check if any of the paths in the event match our target file
                for path in &event.paths {
                    if args.verbose {
                        println!("Comparing {} with {}", path.display(), file_path.display());
                    }
                    
                    if path == &file_path {
                        if args.verbose {
                            println!("Path match found!");
                        }
                        
                        // Only process certain event types: create, modify, or remove
                        match event.kind {
                            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                                println!("[{}] File changed: {}", 
                                        Local::now().format("%Y-%m-%d %H:%M:%S"),
                                        file_path.display());
                                
                                // Execute custom logic here
                                handle_file_change(&file_path, &args.command);
                            },
                            _ => {
                                if args.verbose {
                                    println!("Ignoring event type: {:?}", event.kind);
                                }
                            }
                        }
                    }
                }
            }
            Ok(Err(e)) => eprintln!("Watch error: {:?}", e),
            Err(e) => {
                eprintln!("Error receiving event: {:?}", e);
                break;
            }
        }
    }
}

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