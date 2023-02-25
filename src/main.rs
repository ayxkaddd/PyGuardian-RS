use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn main() -> Result<(), io::Error> {
    // Get the path to the Python file from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: {} <python_file_path>", args[0]);
    }
    let path = &args[1];

    // Run the Python file once before starting the loop
    let mut child = Command::new("python")
        .arg(&path)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()?;
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    println!("Input here");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    writeln!(stdin, "{}", input)?;

    // Get the initial modification time of the file
    let mut last_modified = fs::metadata(&path)?.modified()?;

    loop {
        // Wait for 1 second
        sleep(Duration::from_secs(1));

        // Check if the file has been modified
        let modified = fs::metadata(&path)?.modified()?;
        if modified != last_modified {
            // The file has been modified, so run it
            let mut child = Command::new("python")
                .arg(&path)
                .stdin(Stdio::piped())
                .stdout(Stdio::inherit())
                .spawn()?;
            let stdin = child.stdin.as_mut().expect("Failed to open stdin");
            println!("Input here");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            writeln!(stdin, "{}", input)?;
        
            // Update the modification time of the file
            last_modified = modified;
        }
    }
}

