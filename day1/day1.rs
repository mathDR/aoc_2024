use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    // File test.txt (resp. input.txt) must exist in the current path
    if let Ok(lines) = read_lines("./test.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}