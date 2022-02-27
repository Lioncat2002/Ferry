use std::fs;
use std::process::Command;
use toml::Value;

fn new_project(project_name: String) {
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
    let res = Command::new("python")
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
    let contents="print('Hello World!')";
    fs::write(path, contents).expect("Couldn't write main.py file");
}

fn install_deps() {
    let content = fs::read_to_string("ferry.toml").expect("Damn something went wrong :/");
    let value = content.parse::<Value>().unwrap();
    let dependencies = value["dependencies"].as_table().unwrap();
    
    
    for (name, version) in dependencies {
        let lib = format!("{}=={}", name, version); //format the name and version
        let lib = lib.replace("\"", ""); //need to replace the extra " " coz pip starts acting weird but ok ig

        println!("Installing {}", &lib);
        
        let res = Command::new("env/Scripts/pip")//using the pip inside the virtual env
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
