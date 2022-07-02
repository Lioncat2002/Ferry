use std::fs;
use std::process::Command;

pub fn new_project(project_name: String) {
    //creating the base of ferry.toml
    let contents = format!(
        "[config]
name=\"{}\"
author=\"\"
url=\"\"
[dependencies]
",
        project_name
    );
    //path of the env
    let project_path = format!("{}/env", project_name);
    //creating a new virtual environment
    let res = Command::new("python3")
        .args(["-m", "venv", &project_path])
        .output()
        .expect("Failed to spawn a new virtual env");

    //printing possible outputs and errors
    println!("{}", String::from_utf8_lossy(&res.stdout));
    println!("{}", String::from_utf8_lossy(&res.stderr));

    //creating the ferry.toml
    let path = format!("{}/ferry.toml", project_name);
    fs::write(path, contents).expect("Couldn't write ferry.toml file");

    //creating the main.py
    let path = format!("{}/main.py", project_name);
    let contents = "print('Hello from Ferry!')";
    fs::write(path, contents).expect("Couldn't write main.py file");

    //creating the .gitignore
    let path = format!("{}/.gitignore", project_name);
    let contents = "#ignore the virtual env\n/env";
    fs::write(path, contents).expect("Couldn't write .gitignore file");
}
