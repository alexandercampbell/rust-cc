
use source::File;

pub fn preprocess(file: &mut File) {
    file.lines = file.lines.iter().filter(|&line| {
        !line.starts_with("#")
    }).cloned().collect();
}

