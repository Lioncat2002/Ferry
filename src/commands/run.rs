use std::process::Command;


#[cfg(target_os = "linux")]
static PYTHON_PATH: &str = "env/bin/python3";
#[cfg(target_os = "windows")]
static PYTHON_PATH: &str = "env/Scripts/python";

pub fn run_program() {
    let res = Command::new(PYTHON_PATH) //using the pip inside the virtual env
        .args(["main.py"])
        .output()
        .expect("Failed to run");
    //print out possible outputs and errors
    println!("{}", String::from_utf8_lossy(&res.stdout));
    println!("{}", String::from_utf8_lossy(&res.stderr));
}