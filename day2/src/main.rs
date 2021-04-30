use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp::{min};
fn lines2boxes<'a>(lines: &Vec<String>) -> Vec<Vec<i32>> {
    // Vector<String> -> Vec<Vec<String>> -> Vec<Vec<i32>>
    lines.iter().map::< Vec<i32>, fn(&String)->Vec<i32> >(|line| line.split("x").map(|s| s.parse::<i32>().unwrap()).collect()).collect()
}

fn boxes2paper(boxes: &Vec<Vec<i32>>) -> Vec<i32> {
    boxes.iter().map(|dim| {
        let mut sorted = dim.clone();
        sorted.sort();
        let giftsq = sorted[0] * sorted[1];
        2*dim[0]*dim[1] + 2 * dim[1]*dim[2] + 2*dim[0]*dim[2] + giftsq
    }).collect()
}

fn boxes2ribbon(boxes: &    Vec<Vec<i32>>) -> Vec<i32> {
    boxes.iter().map(|dim| {
        // First ribbon  
                    //    h          d
        let ribbon1 = 2*dim[0] + 2*dim[1];
                    //    h           w
        let ribbon2 = 2*dim[0] + 2*dim[2];
                    //    h          w
        let ribbon3 = 2*dim[0] + 2*dim[2];
                    //   d
        let ribbon4 = 2*dim[1] + 2*dim[2];

        let flower = dim.iter().fold(1, |prod, x| prod * x);
        min(min(min(ribbon1, ribbon2), ribbon3), ribbon4) + flower
    }).collect()
}
fn main() -> io::Result<()>{
    let fh = File::open("input.txt")?;
    let reader = BufReader::new(fh);
    let mut lines: Vec<String> = vec![];
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    
    let boxes = lines2boxes(&lines);
    let paperarea = boxes2paper(&boxes);
    let areatotal = paperarea.iter().fold(0, |area, x| area + x );
    println!("total area: {}", areatotal);

    let ribbontotal = boxes2ribbon(&boxes).iter().fold(0, |total, x| total + x);
    println!("ribbon total: {}", ribbontotal);
    Ok(())
}


