use std::fs;
use std::process::Command;
use toml::Value;
#[cfg(target_os = "linux")]
static PIP_PATH: &str = "env/bin/pip3";
#[cfg(target_os = "windows")]
static PIP_PATH: &str = "env/Scripts/pip";
pub fn install_deps() {
    let content = fs::read_to_string("ferry.toml").expect("Damn something went wrong :/");
    let value = content.parse::<Value>().unwrap(); //parsing the toml file
    let dependencies = value["dependencies"].as_table().unwrap();

    let mut all_deps = vec![];

    for (name, version) in dependencies {
        let lib = format!("{}=={}", name, version.to_string()); //format the name and version
        let lib = lib.replace("\"", ""); //need to replace the extra " " coz pip starts acting weird but ok ig
        all_deps.push(lib); //adding all the dependencies into a string
    }

    //all_deps = all_deps.trim().to_string(); //trimming whitespaces
    //let all_deps = all_deps.split(' ').collect::<Vec<&str>>(); //splitting since pip adds an
    // extra ' ' around the string if pass as a single argument

    println!("Installing:\n{:?}", all_deps);
    //Reduced to a single subprocess call
    let res = Command::new(PIP_PATH) //using the pip inside the virtual env
        .args(["install"])
        .args(all_deps) //pip adds an ' ' around the all_deps string so we are passing it as an array
        .output()
        .expect("Failed to run");

    //print out possible outputs and errors
    println!("{}", String::from_utf8_lossy(&res.stdout));
    println!("{}", String::from_utf8_lossy(&res.stderr));
}