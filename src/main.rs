use std::env;
use std::vec::Vec;
use std::fs;
use std::io::{BufRead, BufReader, Error};
use regex::{Captures, Regex};
use lazy_static::lazy_static;

lazy_static! {
    static ref R: Regex = Regex::new(r#"[ ]+([0-9a-f]+):\t(([0-9a-f]+[ ])+)[ \t]+([\w.]+)\t([^\n]+)"#).unwrap();
    static ref V: Regex = Regex::new(r#"([ \t]*<[^\n]+>[ \t]*)"#).unwrap();
}

fn main() -> Result<(), Error> {
    let args = env::args().collect::<Vec<String>>();
    assert_eq!(args.len(), 2);
    let filepath = &args[1];
    // println!("{}", filepath);

    let file = fs::File::open(filepath)?;
    for line in BufReader::new(file).lines().flatten() {
        if let Some(caps) = R.captures(line.as_str()) {
            let addr = caps.get(1).unwrap().as_str();
            let bytes = caps.get(2).unwrap().as_str();
            let bytes_trim = bytes.replace(' ', "");
            let inst = caps.get(4).unwrap().as_str();
            let operands = caps.get(5).unwrap().as_str();

            let operands_ = V.replace_all(operands, |caps: &Captures| {
                // println!("{:?}", caps.get(1).unwrap());
                ""
            });

            println!("d \"{} {}\" {} {} ()", inst, operands_, bytes_trim, addr);
        }
    }
    Ok(())
}
