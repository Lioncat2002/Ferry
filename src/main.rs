use std::fs;
use std::process::Command;
use toml::Value;

fn new_project(project_name: String) {
    let contents = format!(
        "[config]
name=\"{}\"
[dependencies]
",
        project_name
    );
    let res = Command::new("python")
        .args(["-m", "venv", &project_name])
        .output()
        .expect("Failed to spawn a new virtual env");
    println!("{}", String::from_utf8_lossy(&res.stdout));
    println!("{}", String::from_utf8_lossy(&res.stderr));
    let path = format!("{}/ferry.toml", project_name);
    fs::write(path, contents).expect("Couldn't write file");
}

fn install_deps() {
    let content = fs::read_to_string("ferry.toml").expect("Damn something went wrong :/");
    let value = content.parse::<Value>().unwrap();
    let dependencies = value["dependencies"].as_table().unwrap();
    for (name, version) in dependencies {
        let lib = format!("{}=={}", name, version); //format the name and version
        let lib = lib.replace("\"", ""); //need to replace the extra " " coz pip starts acting weird but ok ig

        println!("{}", &lib);
        let res = Command::new("pip")
            .args(["install", &lib]) //kinda weird but need to add args seperately
            .output()
            .expect("Failed to run"); //calling pip
                                      //print out possible outputs and errors
        println!("{}", String::from_utf8_lossy(&res.stdout));
        println!("{}", String::from_utf8_lossy(&res.stderr));
    }
}

fn main() {
    let command = std::env::args().nth(1).unwrap();
    println!("{}", command);
    if command == "fetch" {
        install_deps();
    }
    if command == "new" {
        let project_name = std::env::args()
            .nth(2)
            .expect("Error missing project name!");
        new_project(project_name);
    }
}
