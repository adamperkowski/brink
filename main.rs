use std::process::Command;
use std::io;
use std::io::Write;
use std::{thread, time::Duration};

fn main() {
    bsh();
    println!();

    let mut inpt = String::new();
    
    while inpt != "exit" {
        inpt = String::new();
        print!("brink < ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inpt).unwrap();

        inpt.pop();

        if inpt == "next" {
            Command::new("playerctl")
                .arg("next")
                .spawn()
                .expect("  [ ! ] Failed to connect!");

            thread::sleep(Duration::from_millis(1000));

            bsh();
            println!("\n  [ * ] Playing next.\n");
        }
        else if inpt == "prv" {
            Command::new("playerctl")
                .arg("previous")
                .spawn()
                .expect("  [ ! ] Failed to connect!");

            thread::sleep(Duration::from_millis(500));

            Command::new("playerctl")
                .arg("previous")
                .spawn()
                .expect("  [ ! ] Failed to connect!");

            thread::sleep(Duration::from_millis(500));

            bsh();
            println!("\n  [ * ] Playing previous. (may not work 100%)\n");
        }
        else if inpt == "back" {
            Command::new("playerctl")
                .arg("previous")
                .spawn()
                .expect("  [ ! ] Failed to connect!");

            thread::sleep(Duration::from_millis(1000));

            bsh();
            println!("\n  [ * ] Playing back.\n");
        }
        else { println!(); }
    }
    
    clrs();
    println!("  [ * ] We will miss you...");
}

fn clrs() {
    Command::new("clear")
        .spawn()
        .expect("  [ ! ] Failed to connect!");
}

fn bsh() {
    Command::new("bash")
        .arg("fetch_cover.sh")
        .spawn()
        .expect("  [ ! ] Failed to connect!")
        .wait()
        .unwrap();
}
