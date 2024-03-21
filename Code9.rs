use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {

    let file = File::open("aakash.txt")?;
    let reader = BufReader::new(file);
    let mut line_count = 0;
    
    for _line in reader.lines() {
        line_count += 1;
    }
    
    println!("Number of lines are {}", line_count);
    Ok(())
}
