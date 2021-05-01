use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

const PART : u8 = 2;
// Part 1
fn double_check<T: AsRef<str>>(s: T) -> bool {
    // Generate regex
    let mut double_re = ('a'..'{') // '{' is character after z...
        .fold::<String, fn(String, char) -> String>
        (String::new(), |str, ch| 
            format!("{}{}{}|", str, ch, ch));
    double_re.pop();
    let re = Regex::new(double_re.as_str()).unwrap();
    //println!("double_check({}) = {}", s.as_ref(), re.is_match(s.as_ref()));
    return re.is_match(s.as_ref());
}

fn vowels_check<T: AsRef<str>>(s: T) -> bool {
    let vowel_count = s.as_ref().chars().filter(|c| {
        match c {
            'a'| 'e' | 'i' | 'o' | 'u' => true,
            _ => false 
        }
    }).count();
    //println!("vowel count({}) = {}", s.as_ref(), vowel_count);
    return vowel_count >= 3;
}

fn disallowed_check<T: AsRef<str>>(s: T) -> bool {
    let re = Regex::new("ab|cd|pq|xy").unwrap();
    let has_bad = re.is_match(s.as_ref());
    //println!("has_bad({}) = {}", s.as_ref(), has_bad);
    return !has_bad;
}


// Part 2
fn twice_pairs<T: AsRef<str>>(s: T)  -> bool {

    // Get all pairs
    let mut pairs : Vec<&str> = vec![];
    for (i,_) in (0..(s.as_ref().as_bytes().len() - 1)).enumerate() {
        pairs.push(&(s.as_ref()[i..i+2]));
    }

    // loop through all pairs looking to see if appears twice
    // with no overlap;
    // skip last pair, since we should already see it earlier if matches
    pairs.pop();
    for x in pairs {
        let first_index = s.as_ref().find(x);
        if let Some(i) = first_index {
            if let Some(_) = s.as_ref()[i+2..].find(x) {
                return true;
            }
        } 
    }
    return false;

}

fn has_repeat<T: AsRef<str>>(s: T) -> bool {

    for i in 0..s.as_ref().len()-2 {
        if s.as_ref().as_bytes()[i] == s.as_ref().as_bytes()[i+2] {
            return true;
        }
    }
    return false;
}

fn main()  -> io::Result<()> {
    let fh = File::open("input.txt")?;
    let reader = BufReader::new(fh);
    let mut lines: Vec<String> = vec![];
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    // examples
    // let mylines = ["ugknbfddgicrmopn", "aaa", "jchzalrnumimnmhp", "haegwjzuvuyypxyu", "dvszwmarrgswjxmb"];
    
    // let nice_str_count_2 = mylines.iter().filter(|s| 
    //     double_check(s) &&
    //     vowels_check(s) &&
    //     disallowed_check(s)).count();

    // println!("number of nice strings: {}", nice_str_count_2);

    if PART == 1 {
    let nice_str_count = lines.iter().filter(|s| 
        double_check(s) &&
        vowels_check(s) &&
        disallowed_check(s)).count();

        println!("number of nice strings: {}", nice_str_count);
    } else {
        let nice_str_count = lines.iter().filter(|s| {
            let t = twice_pairs(s);
            let h = has_repeat(s);
            t && h
        }).count();
        
        println!("number of nice strings for part 2: {}", nice_str_count);
    }
    

    Ok(())
    
}
