use std::env;
use std::fs;

fn main() {
    // let reversed = "\u{001b}[7m";
    // let red = "\u{001b}[31m";
    // let green = "\u{001b}[32m;1m";
    // let yellow = "\u{001b}[33m";
    // let blue = "\u{001b}[34m";
    // let white = "\u{001b}[37m";
    let bold = "\u{001b}[1m";
    let magenta = "\u{001b}[35m";
    let cyan = "\u{001b}[36m";
    let end = "\u{001b}[0m";

    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let filename = &args[2];

    let contents = fs::read_to_string(filename)
        .expect("Something wnt wrong reading the file");


    println!("{:?}", args);
    println!("Searching for: {cyan}{bold}{}{end}", query);
    println!("In file: {magenta}{bold}{}{end}", filename);
    println!("File content:\n\n{}", contents);

}
