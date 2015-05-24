
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
    /*
     * Load a File from the location you specify.
     */
    pub fn from_disk(location: &Path) -> Result<Self, Box<Error>> {
        let mut f = try!(fs::File::open(location));
        let mut s = String::new();
        try!(f.read_to_string(&mut s));

        // Handle block comments
        s = File::strip_block_comments(s);

        // Split the source into lines and process each line separately.
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

    /*
     * Remove all comments from the string that are of the form `/* some comment */`. These may
     * span multiple newlines. Nested block comments are not supported (in the traditional C
     * style).
     */
    fn strip_block_comments(s: String) -> String {
        let mut in_block_comment = false;
        let mut processed = String::with_capacity(s.len());
        let mut iter = s.chars().peekable();

        loop {
            // this is really a manual version of `for c in iter` but we have to be able to peek
            // and skip.
            let c = match iter.next() {
                Some(c) => c,
                None => break,
            };

            if in_block_comment {
                if c == '*' {
                    match iter.peek() {
                        Some(&'/') => {
                            iter.next(); // skip over the `/` character
                            in_block_comment = false;
                        },
                        _ => (),
                    }
                }
                continue;
            }

            // possibly starting new comment
            if c == '/' {
                match iter.peek() {
                    Some(&'*') => {
                        iter.next(); // skip over the `*` character
                        in_block_comment = true;
                        continue;
                    },
                    _ => (),
                }
            }

            processed.push(c);
        }

        processed
    }
}

#[test]
fn strip_block_comments() {
    assert_eq!(File::strip_block_comments(
            "/**/".to_string()),
            "".to_string());

    assert_eq!(File::strip_block_comments(
            "hello /*hello*/goodbye".to_string()),
            "hello goodbye".to_string());

    assert_eq!(File::strip_block_comments(
            "this is my file /* this is a comment */ */".to_string()),
            "this is my file  */".to_string());

    assert_eq!(File::strip_block_comments(
            "this /*is my */file /* this is a\n multiline comment */".to_string()),
            "this file ".to_string());
}

