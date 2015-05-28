use std::fs::File;
use  std::io::Read;

struct Version {
    major: i32,
    minor: i32,
    patch: i32,
}

impl Version {
    fn new(&self) -> Version {
        let mut file = match File::open("VERSION") {
            Ok(file) => file,
            Err(..) => panic!("Couldn't find VERSION file"),
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let mut v: Vec<&str> = content.Split('.').collect();
        let test = Version(v[0], v[1], v[2]);
    };
}

fn main() {
    let mut file = match File::open("VERSION") {
        Ok(file) => file,
        Err(..) => panic!("Couldn't find VERSION file"),
    };
    let mut content = String::new();
    let x = file.read_to_string(&mut content).unwrap();
    
    println!("contents: {}", content);
}
