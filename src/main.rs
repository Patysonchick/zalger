use std::{fs, io};

// const SIMBL: char = '\u{0F37}';
const SYMBOL: char = '\u{034F}';
const CO: usize = 10;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    let mut out = String::new();
    io::stdin().read_line(&mut input)?;

    for c in input.trim().chars() {
        out.push(c);
        for _ in 0..CO {
            out.push(SYMBOL);
        }
    }

    println!("{out}");
    println!("\nWritten to out.txt");
    fs::write("out.txt", out).unwrap();

    Ok(())
}
