//tutorial-setup-01.rs
// Import the standard library's I/O module so we can read from stdin.
use std::io;

fn main() {
    // Create a CSV parser that reads data from stdin.
    let mut reader = csv::Reader::from_reader(io::stdin());
    // Loop over each record.
    for result in reader.records() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record = result.expect("a CSV record");
        // Print a debug version of the record.
        println!("{:?}", record);
    }
}
