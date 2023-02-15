extern crate termion;
extern crate textwrap;

use std::fs;
use std::io;
use std::io::Write;
use std::process::Command;

use termion::clear;
use termion::cursor;

pub fn run() -> Result<bool, String> {
    println!("It's the year 3000, you are the latest and greatest robot barista. In the year 3000 all orders are fed to you through a JSON file, its then up to you if the order is valid or not. Luckily you've been modded to be able to use OPA (Open Policy Agent) which will help you set some rules.");
    println!("Press Enter if you are ready!");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    // Clear the terminal and move the cursor to the top-left corner
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    // Flush the output buffer to ensure that the terminal is cleared
    io::stdout().flush().unwrap();

    loop {

        // Clear the screen and move the cursor to the top-left corner
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        // Flush the output buffer to ensure that the terminal is cleared
        io::stdout().flush().unwrap();

        println!(
            "As the proud employee of a specialty coffee shop, you can't believe your robot ears when a customer walks in and asks for a cola, completely disregarding the carefully crafted menu of premium coffee offerings."
        );
        println!("Level 1 Order (level1.json):");

        let file_contents = fs
            ::read_to_string("src/level1/level1.json")
            .expect("Failed to read level1.json");

        let output = Command::new("opa")
            .arg("eval")
            .arg("--format")
            .arg("pretty")
            .arg("--data")
            .arg("src/level1/level1.rego")
            .arg("--input")
            .arg("src/level1/level1.json")
            .arg("data.barista")
            .output()
            .expect("Failed to execute opa command");

        // convert stdout from bytes to string
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Print a border around the JSON output
        println!("{}", termion::color::Fg(termion::color::Green));
        println!("┌{}┐", "─".repeat(56));
        for line in file_contents.lines() {
            println!("│ {: <54} │", line);
        }
        println!("└{}┘", "─".repeat(56));
        println!("{}", termion::color::Fg(termion::color::Reset));

        // Print a border around the rego output
        println!("{}", termion::color::Fg(termion::color::Red));
        println!("┌{}┐", "─".repeat(56));
        for line in stdout.lines() {
            println!("│ {: <54} │", line);
        }
        println!("└{}┘", "─".repeat(56));
        println!("{}", termion::color::Fg(termion::color::Reset));

        if stdout.contains("Unfortunately, we do not serve Cola") {
            return Ok(true);
        } else {
            println!("Hmmm... something doesn't seem right. Try again?");
            println!("1. Retry");
            println!("2. Exit");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            match input.trim() {
                "1" => {}
                "2" => {
                    return Ok(false);
                }
                _ => {}
            }
        }
    }
}