mod smbls;

use crate::smbls::Input;
use std::{fs, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    // let mut out = String::new();
    io::stdin().read_line(&mut input)?;

    let mut input = Input::new(input);
    input.smlr_depth();

    let out = input.obfs();

    println!("{out}");
    println!("\nWritten to out.txt");
    fs::write("out.txt", out).unwrap();

    Ok(())
}
