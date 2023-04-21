use std::process::Command;

fn main() {
    let output = Command::new("cmd")
    .args(&["/C","src/programs.bat"])
    .output()
    .expect("failed");

    for out in String::from_utf8(output.stdout).iter() {
        println!("{}", out);
    }
}

// want a command line program. 
// user types in the file locations.
// pastes them into the batch file and then runs the file.
//add image it should use.