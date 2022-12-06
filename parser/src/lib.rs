use std::path::Path;
use std::fs::File;
use std::str::FromStr;
use std::io::BufReader;
use std::io::BufRead;

pub fn parse(filename: &str) -> Vec<String> {
    // println!("{:?}", std::env::current_dir());
    let mut string = match String::from_str("./input/") {
        Ok(s) => s,
        Err(_e) => return Vec::new(),
    };
    string += filename;
    string += ".txt";
    let path = Path::new(&string);

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let file = BufReader::new(file);
    let result: Result<Vec<String>, std::io::Error> = file.lines().collect();
    return result.unwrap();
}
