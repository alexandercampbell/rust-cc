
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;

/*
 * File is our initial datastructure. We load the source code into this structure and then do
 * operations on it.
 */
pub struct File {
    // yeah it's expensive memory-wise, but I'm lazy
    pub lines: Vec<String>,
    pub name:  String,
}

impl File {
    pub fn from_disk(location: &Path) -> Result<Self, Box<Error>> {
        let mut f = try!(fs::File::open(location));
        let mut s = String::new();
        try!(f.read_to_string(&mut s));

        // TODO: block comment handling needs to happen right here.

        let mut lines = vec![];
        for line in s.split("\n") {
            lines.push(line.to_string());
        }

        Ok(File{
            lines: lines,
            // NOTE: this unwrap ignores unicode edge cases. Probably dangerous.
            name: (*location).to_str().unwrap().to_string(),
        })
    }
}

