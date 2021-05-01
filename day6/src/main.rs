use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::cmp::{min, max};
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
#[derive(Debug)]
enum Operation {
    On((i32,i32), (i32,i32)),
    Off((i32,i32), (i32,i32)),
    Toggle((i32,i32), (i32,i32))
}
#[derive(Debug)]
enum ParseError {
    Err(String)
}

fn parse_line(s: &str) -> Result<Operation, ParseError> {
    let v: Vec<&str> = s.split(&[' ', ','][..]).collect();
    match v[0] {
        "turn" => {
            match v[1] {
                "on" => {
                return Ok(Operation::On((v[2].parse::<i32>().unwrap(),
                                  v[3].parse::<i32>().unwrap()),
                                 (v[5].parse::<i32>().unwrap(),
                                  v[6].parse::<i32>().unwrap())))
                },
                "off" => {
                return Ok(Operation::Off((v[2].parse::<i32>().unwrap(),
                    v[3].parse::<i32>().unwrap()),
                   (v[5].parse::<i32>().unwrap(),
                    v[6].parse::<i32>().unwrap())))
                },
                _ => {
                    return Err(ParseError::Err("turn parse error".to_string()));
                }
            }
        },
        "toggle" => {
            return Ok(Operation::Toggle((v[1].parse::<i32>().unwrap(),
                                         v[2].parse::<i32>().unwrap()),
                                        (v[4].parse::<i32>().unwrap(),
                                        v[5].parse::<i32>().unwrap())))
        },
        _ => {
            return Err(ParseError::Err("no turn or toggle error".to_string()));
        }
    }
}

fn do_operation(grid:  &mut[[bool;1000]; 1000], op: Operation) {

    let (start_x, start_y, end_x, end_y) : (i32, i32, i32, i32);
    match op {
        Operation::On((x1,y1),(x2,y2)) => {
            start_x = min(x1,x2);
            end_x = max(x1,x2);
            start_y = min(y1,y2);
            end_y = max(y1,y2);
        
        },
        Operation::Off((x1,y1),(x2,y2)) => {
            start_x = min(x1,x2);
            end_x = max(x1,x2);
            start_y = min(y1,y2);
            end_y = max(y1,y2);
            
        },
        Operation::Toggle((x1,y1),(x2,y2)) => {
            start_x = min(x1,x2);
            end_x = max(x1,x2);
            start_y = min(y1,y2);
            end_y = max(y1,y2);
        }
    }
    for x in start_x..(end_x+1) {
        for y in start_y..(end_y+1) {
            match op {
                Operation::On(_,_) => {
                    grid[x as usize][y as usize] = true;
                },
                Operation::Off(_,_) => {
                    grid[x as usize][y as usize] = false;
                },
                Operation::Toggle(_,_) => {
                    grid[x as usize][y as usize] = !grid[x as usize][y as usize];
                }
            }
        }
    }
}

fn count_lights(grid:  &[[bool;1000]; 1000]) -> i32 {
    // let mut cnt = 0;
    // // Can I use flat_map() and fold to do this??
    // for x in 0..1000 {
    //     for y in 0..1000 {
    //         if grid[x][y] {
    //             cnt += 1;   
    //         }
    //     }
    // }
    grid.iter().flat_map(|s| s.to_vec()).fold(0, |total, b| total + match b { true=>1, false=>0})
}

fn main() -> io::Result<()> {
    let lines = read_file("input.txt")?;
    let mut grid : [[bool; 1000]; 1000] = [[false; 1000]; 1000]; 
    //println!("lines: {:?}", lines);
    let operations = lines.iter()
        .map::<Operation, fn(&String)->Operation>(|l|
            parse_line(l.as_str()).unwrap());
    operations.for_each(|o| do_operation(&mut grid, o));
    println!("There are {} lights on", count_lights(&grid));
    Ok(())
}
