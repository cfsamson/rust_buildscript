use std::fs::*;
use std::io::Write;
use std::path::*;
use std::process::Command;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cur_dir = std::env::current_dir()?;
    // delete output folder if it exists
    if cur_dir.join("output").exists() {
        println!("Deleting output directory...");
        std::fs::remove_dir_all(cur_dir.join("output"))?; 
    }

    // create a output folder
    println!("Creating output dir...");
    let dir = DirBuilder::new();
    dir.create(cur_dir.join("output"))?;

    println!("Running cargo build...");
    // run cargo build
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cargo build --release"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .args(&["-c", "cargo build --release"])
            .output()
            .expect("failed to execute process")
    };
    std::io::stdout().write_all(&output.stdout)?;
    std::io::stderr().write_all(&output.stderr)?;

    println!("Copying files...");
    // copy the binary and the config
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", r#"copy target\release\make.exe && copy src\config.txt"#])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .args(&["-c", "cp ./target/release/make ./output && cp ./src/config.txt ./output"])
            .output()
            .expect("failed to execute process")
    };
    std::io::stdout().write_all(&output.stdout)?;
    std::io::stderr().write_all(&output.stderr)?;
    println!("Finished!");

    Ok(())
}
