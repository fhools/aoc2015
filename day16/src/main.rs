use std::collections::HashMap;
const PART : u8 = 2;

use std::io::prelude::*;
use std::io;
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


#[derive(Debug, Clone)]
struct Aunty {
    id: i32,
    props: Vec<Option<i32>>,
}


impl Aunty {
    fn new(id: i32) -> Self {
        Aunty {
            id,
            props: vec![None; 10]
        }
    }
    fn add(&mut self, k: &String, val: i32, prop2id: &HashMap<String,i32>) {
        let keyid = prop2id.get(k).unwrap();
        self.props[*keyid as usize] = Some(val);
    }

    fn is_target(&self, target_props: &Vec<i32>) -> bool {
        for i in 0..target_props.len() {
            if let Some(val) = self.props[i] {
                // cats, trees should be greater than
                if (PART == 2) && (i == 1 || i == 7) {
                    if val <= target_props[i]  {
                        return false;
                    }
                } else if (PART == 2) && (i == 3 || i == 6) {
                // pomeranians, goldfishes should be less than
                    if val >= target_props[i] {
                        return false;
                    }
                // all others should be equal to target property
                } else {
                    if val != target_props[i] {
                        return false;
                    }
                }
            } 
        }
        return true;
    }
}

fn parseline(s: &String, prop2id: &HashMap<String,i32>) -> Aunty {
   // example input
   // Sue 1: cars: 9, akitas: 3, goldfish: 0
    let mut ss = s.clone();
    let firstcolon = ss.find(":");
    if let Some(idx1) = firstcolon {
        let props_str = ss.split_off(idx1)[1..].to_string();
        let idtoks: Vec<&str> = ss.split(" ").collect();
        let id = idtoks[1].trim().to_string().parse::<i32>().unwrap();
        let mut aunty = Aunty::new(id);
        let propstoks : Vec<&str>= props_str.split(",").collect();
        for x in propstoks.iter() {
            let propnumtoks : Vec<&str>  = x.split(":").collect();
            let propkey = propnumtoks[0].trim();
            let propval = propnumtoks[1].trim().to_string().parse::<i32>().unwrap();
            aunty.add(&propkey.to_string(), propval, prop2id);
            
        }
        return aunty;
    }   
    Aunty::new(1000)
}
fn main() {
    let mut prop2id : HashMap<String, i32> = HashMap::new();
    // We're mapping each property to an id so we can iterate through
    // an array of properties. 
    // Create the key to id map here.
    prop2id.insert("children".to_string(), 0);
    prop2id.insert("cats".to_string(), 1);
    prop2id.insert("samoyeds".to_string(), 2);
    prop2id.insert("pomeranians".to_string(), 3);
    prop2id.insert("akitas".to_string(), 4);
    prop2id.insert("vizslas".to_string(), 5);
    prop2id.insert("goldfish".to_string(), 6);
    prop2id.insert("trees".to_string(), 7);
    prop2id.insert("cars".to_string(), 8);
    prop2id.insert("perfumes".to_string(), 9);

    // Values we are looking for for each property
    let target_props = vec![3, 7, 2, 3, 0, 0, 5, 3, 2, 1];
   
    let lines = read_file("input.txt").unwrap();
    let aunty : Vec<&String> = lines.iter().filter(|l| {
        let aunty = parseline(l, &prop2id);
        aunty.is_target(&target_props)
    }).collect();
    println!("aunty: {:?}", aunty);
}
