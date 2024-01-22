use std::env;
use std::fmt::{Display, Formatter};
use std::vec::Vec;
use std::fs;
use std::io::{BufRead, BufReader, Error};
use regex::{Captures, Regex};
use lazy_static::lazy_static;

lazy_static! {
    static ref R: Regex = Regex::new(r#"[ ]+([0-9a-f]+):\t(([0-9a-f]+[ ])+)[ \t]+([\w.]+)\t?([^\n]+)?"#).unwrap();
    static ref R_: Regex = Regex::new(r#"[ ]+([0-9a-f]+):\t(([0-9a-f]+[ ])+)"#).unwrap();
    static ref V: Regex = Regex::new(r#"([ \t]*<[^\n]+>[ \t]*)"#).unwrap();
}

struct Instruction {
    addr: u64,
    bytes: String,
    inst: String,
    operands: Option<String>,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(operands) = &self.operands {
            write!(f, "d \"{} {}\" {} {:#08x}", self.inst, operands, self.bytes, self.addr)
        } else {
            write!(f, "d \"{}\" {} {:#08x}", self.inst, self.bytes, self.addr)
        }
    }
}

fn main() -> Result<(), Error> {
    let args = env::args().collect::<Vec<String>>();
    assert_eq!(args.len(), 2);
    let filepath = &args[1];
    // println!("{}", filepath);

    let file = fs::File::open(filepath)?;
    let mut insts = vec![];
    for line in BufReader::new(file).lines().flatten() {
        if let Some(caps) = R.captures(line.as_str()) {
            let addr = caps.get(1).unwrap().as_str();
            let addr = u64::from_str_radix(addr, 16).expect("Failed to parse addr");
            let bytes = caps.get(2).unwrap().as_str();
            let bytes_trim = bytes.replace(' ', "");
            let inst = caps.get(4).unwrap().as_str();
            let operands = caps.get(5).map(
                |cap| {
                    V.replace_all(cap.as_str(), |_caps: &Captures| {
                        // println!("{:?}", caps.get(1).unwrap());
                        ""
                    }).to_string()
                }
            );

            insts.push(Instruction {
                addr,
                bytes: bytes_trim.to_string(),
                inst: inst.to_string(),
                operands,
            });
        } else if let Some(caps) = R_.captures(line.as_str()) {
            let inst = insts.pop().unwrap();
            let bytes = caps.get(2).unwrap().as_str().replace(' ', "");
            let bytes = inst.bytes.clone() + bytes.as_str();
            insts.push(Instruction {
                bytes,
                ..inst
            });
        }
    }

    for inst in insts {
        println!("{}", inst);
    }
    Ok(())
}
