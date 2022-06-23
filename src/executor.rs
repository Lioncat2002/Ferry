use std::fs;
use std::process::Command;
use regex::Regex;
use toml::Value;

#[cfg(target_os = "linux")]
static PIP_PATH: &str = "env/bin/pip3";
#[cfg(target_os = "windows")]
static PIP_PATH: &str = "env/Scripts/pip";

#[cfg(target_os = "linux")]
static PYTHON_PATH: &str = "env/bin/python3";
#[cfg(target_os = "windows")]
static PYTHON_PATH: &str = "env/Scripts/python";

pub fn generate_docs(path:String){

    /*
    setting '''doc as a special type of docstring which will be used 
    for generating documentation :p

    also use lazy repetition(.*?) instead of greedy(.*)
    else incorrect selection of docstring contents occures when 
    using multiple docstrings in a single file
    ref: https://users.rust-lang.org/t/regular-expression/56925/4 
    */ 
    
    let re=Regex::new(r"(?s)def(.*?)'''doc(.*?)'''").unwrap();

    let data=fs::read_to_string(path).expect("Error file not found!");

    for doc in re.captures_iter(&data){
        println!("Function definition:");
        println!("{}\n",doc.get(1).unwrap().as_str().trim());
        println!("Function description:");
        println!("{}\n\n",doc.get(2).unwrap().as_str().trim());
        //TODO: Write the definition and description to a html file      
    }

}


pub fn run_program(){

    let res = Command::new(PYTHON_PATH) //using the pip inside the virtual env
        .args(["main.py"])
        .output()
        .expect("Failed to run");
    //print out possible outputs and errors
    println!("{}", String::from_utf8_lossy(&res.stdout));
    println!("{}", String::from_utf8_lossy(&res.stderr));
}

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
    let res = Command::new(PIP_PATH) //using the pip inside the virtual env
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
    let contents = "#ignore the virtual env\nenv/";
    fs::write(path, contents).expect("Couldn't write .gitignore file");
}
