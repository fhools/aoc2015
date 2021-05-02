use std::str;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_file(s: &str) -> Result<Vec<String>, io::Error> {
    let fh = File::open(s);
    if let Ok(fh) = fh {
        let reader = BufReader::new(fh);
        return Ok(reader.lines().collect::<Result<_,_>>().unwrap());
    }  else if let Err(err) = fh {
        return Err(err);
    } else {
        return Err(io::Error::new(io::ErrorKind::Other,"read_file fail"));
    }
}

fn formatstr(s: &str) -> Vec<u8> {
    let mut i : usize = 0;
    // remove starting and trailing '"'
    let ss = s[1..s.len()-1].as_bytes();
    let mut data : Vec<u8> = vec![];
    while i < ss.len() {
        match ss[i] as char {
            '\\' => {
                match ss[i+1] as char {
                    'x' => {
                        let hex = &ss[i+2..i+4];
                        let d = u8::from_str_radix(str::from_utf8(hex).unwrap(), 16).unwrap();
                        data.push(d);
                        i += 4;
                    },
                    '\\' => {
                        data.push('\\' as u8);
                        i += 2;
                    },
                    '"' => {
                        data.push('#' as u8);
                        i += 2;
                    },
                    _ => {
                        println!("unknown escape char {}", ss[i+1] as char);
                        return data;
                    }
                }
            },
            _ => {
                data.push(ss[i] as u8);
                i += 1;
            }
        }
    }
    return data;
}
fn main() {
    let inputs = read_file("input.txt").unwrap();
    let total_delta : usize = inputs.iter().map(|l| {
        let result = formatstr(l);
        let delta = l.len() - result.len();
        return delta;
    }).sum();
    println!("total difference: {}", total_delta);
    // let input = r#""\xff"#;
    // let result = formatstr(input);
    // println!("format: {:?}", result);
}
