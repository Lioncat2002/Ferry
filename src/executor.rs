use std::fs;
use std::process::Command;
use toml::Value;

pub fn install_deps() {
    let content = fs::read_to_string("ferry.toml").expect("Damn something went wrong :/");
    let value = content.parse::<Value>().unwrap(); //parsing the toml file
    let dependencies = value["dependencies"].as_table().unwrap();

    let mut all_deps = "".to_owned();

    for (name, version) in dependencies {
        let lib = format!("{}=={} ", name, version); //format the name and version
        let lib = lib.replace("\"", ""); //need to replace the extra " " coz pip starts acting weird but ok ig
        all_deps.push_str(&lib); //adding all the dependencies into a string
    }

    all_deps = all_deps.trim().to_string(); //trimming whitespaces
    let all_deps = all_deps.split(' ').collect::<Vec<&str>>(); //splitting since pip adds an
                                                               // extra ' ' around the string if pass as a single argument

    println!("Installing:\n{:?}", all_deps);
    //Reduced to a single subprocess call
    let res = Command::new("env/Scripts/pip") //using the pip inside the virtual env
        .args(["install"])
        .args(all_deps) //pip adds an ' ' around the all_deps string so we are passing it as an array
        .output()
        .expect("Failed to run");

    //print out possible outputs and errors
    println!("{}", String::from_utf8_lossy(&res.stdout));
    println!("{}", String::from_utf8_lossy(&res.stderr));
}

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
    let contents = "print('Hello World!')";
    fs::write(path, contents).expect("Couldn't write main.py file");

    //creating the .gitignore
    let path = format!("{}/.gitignore", project_name);
    let contents = "#ignore the virtual env\nenv/";
    fs::write(path, contents).expect("Couldn't write .gitignore file");
}