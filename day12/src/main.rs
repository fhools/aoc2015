use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use serde_json::{Result, Value};
const PART: u8 = 2;
fn sum_nums(val: &Value) -> f64 {
    let mut sum = 0.0;
    match val {
        Value::Object(ref map) => {
            let mut foundred = false;

            // Part 2 skips counting any objects (and chidren) if there exists
            // a key with value "red"
            if PART == 2 {
                for v in map.values() {
                    if let Value::String(ref s) =  v {
                        if s == "red" {
                            foundred = true;
                            break;
                        }
                    }                 
                }
            }
            for v in map.values() {
                if PART == 1 || (PART == 2 && !foundred) {
                    sum += sum_nums(v);
                }
            }
        },

        Value::Array(ref arr) => {
            for a in arr {
                sum += sum_nums(a);
            }
        },

        Value::Number(ref num) => {
            sum += num.as_f64().unwrap();
        },

        _ => {

        }

    }
    sum
}
fn main()  {
    
    let fh = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(fh);
    let mut data = String::new();
    if let _ = reader.read_to_string(&mut data) {
        let v: Value = serde_json::from_str(&data).unwrap();
        println!("sum: {}", sum_nums(&v));
    }
}
