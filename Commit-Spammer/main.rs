use std::process::{Command, exit};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};

fn main() {
    let file_name = "file.txt";
    let commit_message = "Mass Commit, No:";

    if fs::metadata(".git").is_ok() {
        println!("Repository Initialised Already");
    } else {
        println!("We would recommend initialising a GitHub repository.");
        println!("Either use \"git initialise\" or use GitHub Desktop.");
        println!("If you would like us to do it for you press y, otherwise press n");

        let mut choice = String::new();
        loop {
            io::stdin().read_line(&mut choice).expect("Failed to read line");
            choice = choice.trim().to_lowercase();
            if choice == "y" {
                println!("Initialising Repository.");
                Command::new("git").args(&["init"]).output().expect("Failed to execute command");
                let mut file = File::create("README.md").expect("Failed to create file");
                file.write_all(b"# Commit").expect("Failed to write to file");
                Command::new("git").args(&["add", "README.md"]).output().expect("Failed to execute command");
                Command::new("git").args(&["commit", "-m", "Initial commit"]).output().expect("Failed to execute command");
                println!("We have initialised the repository. Simply import it to GitHub to use this program.");
                exit(0);
            } else if choice == "n" {
                break;
            } else {
                println!("Please input a single letter. Either Y or N");
            }
        }
    }

    if fs::metadata(file_name).is_ok() {
        println!("File Exists. Continuing");
    } else {
        let mut file = File::create(file_name).expect("Failed to create file");
        file.write_all(b"File Initialised").expect("Failed to write to file");
        println!("File created: {}", file_name);
    }

    print!("Mass Commits, By Kars.\nPlease Input the amount of commits you would like. (Recommended 10-100 for speed)\n");

    let mut commits_str = String::new();
    loop {
        io::stdin().read_line(&mut commits_str).expect("Failed to read line");
        commits_str = commits_str.trim().to_string();
        if let Ok(commits) = commits_str.parse::<usize>() {
            if commits > 0 {
                println!("Starting Mass Commits. Press Control + C at any time to stop the commit creation.");

                for i in 1..commits {
                    let mut file = OpenOptions::new().append(true).open(file_name).expect("Failed to open file");
                    writeln!(file, "Edit {}", i).expect("Failed to write to file");
                    Command::new("git").args(&["add", file_name]).output().expect("Failed to execute command");
                    Command::new("git").args(&["commit", "-m", &format!("{} {}", commit_message, i)]).output().expect("Failed to execute command");
                }

                Command::new("git").args(&["push", "origin", "master"]).output().expect("Failed to execute command");
                break;
            } else {
                println!("Please Input a valid number");
            }
        } else {
            println!("Please Input a Number");
        }
    }

    println!("Thank you for using my Commit thingy");
}
