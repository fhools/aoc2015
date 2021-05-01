use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io;
use std::io::prelude::*;
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


    let nice_str_count = lines.iter().filter(|s| 
        double_check(s) &&
        vowels_check(s) &&
        disallowed_check(s)).count();

    println!("number of nice strings: {}", nice_str_count);

   

    Ok(())
    
}
