use std::fs::File;
use std::io::Write;
use std::io::Read;

pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

impl Version {
    pub fn new() -> Version {
        let mut file = match File::open("VERSION") {
            Ok(file) => file,
            Err(..) => panic!("Couldn't find VERSION file"),
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let v: Vec<&str> = content.split('.').collect();

        Version {
            major: v[0].parse::<i32>().unwrap(),
            minor: v[1].parse::<i32>().unwrap(),
            patch: v[2].parse::<i32>().unwrap()
        }
    }

    pub fn bump_major(&mut self) -> i32 {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
        self.update_version();
        self.major
    }

    pub fn bump_minor(&mut self) -> i32 {
        self.minor += 1;
        self.patch = 0;
        self.update_version();
        self.minor
    }

    pub fn bump_patch(&mut self) -> i32 {
        self.patch += 1;
        self.update_version();
        self.patch
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.major.to_string());
        s.push_str(".");
        s.push_str(&self.minor.to_string());
        s.push_str(".");
        s.push_str(&self.patch.to_string());
        s
    }

    pub fn update_version(&self) -> bool {
        let mut file = match File::create("VERSION") {
            Ok(file) => file,
            Err(..) => panic!("Couldn't update VERSION file"),
        };
        match file.write_all(self.to_string().as_bytes()) {
            Ok(..) => true,
            Err(..) => false
        }
    }
}