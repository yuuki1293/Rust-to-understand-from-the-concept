use std::fs::File;
use std::io;

mod line_read;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;

    let lines_vec = line_read::get_lines(f);

    println!("{:?}", lines_vec);
    Ok(())
}
