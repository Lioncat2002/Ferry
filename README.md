# Ferry
A Rustified package manager for python

# How to Use?
- Make sure you got Rust installed since I am not yet providing a built version :p
- Compile the project with `cargo build`
- Create a new project with `ferry new <project_name>` 
- Add your dependencies along with their versions under the `[dependencies]` heading in `ferry.toml`
- Install all the dependencies with `ferry fetch`

# Features
- Clap Based CLI
- Create new python project with `ferry new <project_name>`
- Install packages from `ferry.toml` using `ferry fetch`
- Run python program with `ferry run`
- Python Docs generation `ferry doc <name_of_file.py>`


# Road Map

### 0.0.3
- [ ] Support for md in the doc strings
- [ ] Convert doc to html
- [x] Linux support
- [x] Ability to generate Docs from the docstrings~

### 0.0.2
- [x] Ability to run the python program using `ferry run`
- [x] Better Cli



