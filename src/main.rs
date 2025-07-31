use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
fn main() {
    //env::args() : This function returns an iterator over the arguments passed to the program during its execution.
    //collect() : This method transforms the iterator into a Vec<String>, a collection of strings
    println!("---- File Reader ----");
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    if args.len() < 2 {
        eprintln!("Usage: cargo run <file_path> [--lines] [--search <keyword>]");
        return;
    }
    //convert &str :  &"--lines" en string
    //true if the element is found in args.
    //vec!["target/debug/json_parser.exe", "example.txt", "--lines"]
    //vec!["target/debug/json_parser.exe", "example.txt", "--search"]

    let show_line = args.contains(&"--lines".to_string());
    //Postition Searches for an element in an iterator, returning its index.
    let keyword = if let Some(pos) = args.iter().position(|x| x == "--search") {
        args.get(pos + 1)
    } else {
        None
    };

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open file :{}", e);
            return;
        }
    };
    //Create a buffered reader for the file, allowing the file to be read line by line more efficiently.
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        //reader.lines() This method returns an iterator that allows you to read the file line by line.
        //eneumerate Adds an index to each line read, which also gets the line number (starting at zero)
        let line = line.unwrap();
        let matched = keyword.map_or(true, |k| line.contains(k)); //It checks if the line contains this keyword. Otherwise, it always returns true
        if matched {
            if show_line {
                println!("{} : {} ", i + 1, line);
            } else {
                println!("{}", line);
            }
        }
    }
    //If the --lines option was specified, the line will be displayed with its line number (i + 1, because indexes start at 0).
}
