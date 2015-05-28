mod version;
use version::Version;
use std::env::args;
use std::convert::AsRef;

fn main() {
    let mut version = Version::new();
    let v: Vec<String> = args().collect();
    println!("{}", v.len());
    if v.len() < 2 {
        panic!("Please specify which part of the VERSION you would like to bump. (major/minor/patch)")
    }
    else {
        let first = v[1].to_string();
        
        match first.as_ref() {
            "major" => version.bump_major(),
            "minor" => version.bump_minor(),
            "patch" => version.bump_patch(),
            _       => panic!("Not a recognized option. Please use 'major', 'minor', or 'patch'.")
        };

        println!("major: {}, minor: {}, patch: {}", version.major, version.minor, version.patch);
    }
}
