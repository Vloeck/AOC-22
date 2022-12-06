use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub(crate) fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let lines = lines.into_iter()
        .filter_map(|e| e.ok())
        .collect();
    Ok(lines)
}
