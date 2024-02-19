use std::process::{Command, Stdio};
use std::io;
use std::io::Write;
use std::{thread, time::Duration};

/* help
   
   next  >  Skip current song.
   back  >  Play back (standard back action)
   prv   >  Play previous song (may not work the best)
   clrs  >  Reload screen (you can also use ↵ ) 
   plpa  >  Play / pause */

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
        else if inpt == "plpa" {
            Command::new("playerctl")
                .arg("play-pause")
                .spawn()
                .expect("  [ ! ] Failed to connect!");

            thread::sleep(Duration::from_millis(500));

            bsh();

            let state = Command::new("playerctl")
                .arg("status")
                .stdout(Stdio::piped())
                .output()
                .expect("  [ ! ] Failed to connect!");

            let mut stdout = String::from_utf8(state.stdout).unwrap();

            stdout.pop();
            
            println!("\n  [ * ] {stdout}.\n");
        }
        
        else if inpt == "clrs" || inpt == "" {
            bsh();
            println!();
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
