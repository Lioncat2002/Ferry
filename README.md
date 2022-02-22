# Ferry
A Rustified package manager for python

# How to Use?
- Make sure you got Rust installed since I am not yet providing a built version :p
- Create a new project with `ferry new <project_name>` 
- Manually activate the virtual environment with `<project_name>/env/Scripts/activate` (kinda facing trouble trying to activate this automatically)
- Add your dependencies along with their versions under the `[dependencies]` heading in `ferry.toml`
- Install all the dependencies with `ferry fetch`

# Road Map
### 0.0.0:
- ~Ability to install dependencies~
- ~Ability to create a new project~

### 0.0.1 (WIP)
- Ability to uninstall dependencies
- Custom Virtual Environment
- More features TBD
