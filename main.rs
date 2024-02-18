use std::process::Command;

fn main() {
    Command::new("bash")
        .arg("fetch_cover.sh")
        .spawn()
        .expect("ls command failed to start")
        .wait()
        .unwrap();
}
