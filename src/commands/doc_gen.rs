use comrak::{markdown_to_html, ComrakOptions};
use regex::Regex;
use std::fs;
pub fn generate_docs(path: String) {
    /*
    setting '''doc as a special type of docstring which will be used
    for generating documentation :p
    (?s) is used for multi line matching
    also use lazy repetition(.*?) instead of greedy(.*)
    else incorrect selection of docstring contents occures when
    using multiple docstrings in a single file
    ref: https://users.rust-lang.org/t/regular-expression/56925/4

    */
    let re = Regex::new(r"(?s)def(.*?)'''doc(.*?)'''").unwrap();

    let data = fs::read_to_string(&path).expect("Error file not found!");

    let mut html = String::new();
    for doc in re.captures_iter(&data) {
        let func_def = doc.get(1).unwrap().as_str().trim();
        let func_desc = doc.get(2).unwrap().as_str().trim();

        let doc = format!("## {}\n {}", func_def, func_desc);//making the function defination as a heading
        
        html += &markdown_to_html(&doc, &ComrakOptions::default());//obtained from comRak
        //TODO: auto write feature for all files
    }
    fs::write("doc.html", html).unwrap();
}