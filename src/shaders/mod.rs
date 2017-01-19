use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::error::Error;

pub fn load(filename: &str) -> String {
    println!("filename {}", filename);
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => return s
    }
}
