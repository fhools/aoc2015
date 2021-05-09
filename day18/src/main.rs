
use std::fmt;

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


const PART : u8 = 2;

#[derive(Debug, Clone)]
struct Board<E> where
E: fmt::Display + fmt::Debug + Clone + Default  + Into<i32>  {
    grid: Vec<Vec<E>>,
    size: usize 
}

impl<E: fmt::Display +  fmt::Debug + Default + Clone + Into<i32>> Board<E>  {
    fn new(sz: usize) -> Board<E> {
        let mut board = Board {
            grid: Vec::new(),
            size: sz
        };

        for _ in 0..board.size {
            let row : Vec<E> = vec![E::default(); board.size];
            board.grid.push(row);
        }
        board
    }

    fn visit_clone<F>(&self, f: F) -> Board<E>
    where 
        F: Fn(i32, i32, &Board<E>) -> E {
        let mut board : Board<E> = self.clone();
        for y in 0..self.size {
            for x in 0..self.size {
                board.grid[y][x] = f(x as i32, y as i32, &self);
            }
        }
        board
    }

    fn print(&self) -> String {
        let mut s = "\n".to_string();
        for row in &self.grid {
            let mut s_row = String::new();
            for c in row {
                s_row.push(c.to_string().chars().nth(0).unwrap());
            }
            s = format!("{}{:?}\n", s, s_row);
        }
        s
    }

    fn get(&self, x: i32, y: i32) -> Option<E> {
        if x < 0 || x >= (self.size as i32)  ||
            y < 0 || y >= (self.size as i32) {
                return None;
        }
        return Some(self.grid[y as usize][x as usize].clone());
    }

    fn set(&mut self, x: i32, y: i32, val: &E) {
        if let Some(ref _unusedval) = self.get(x,y) {
            self.grid[y as usize][x as usize] = val.clone();
        } 
    }

    fn count_on(&self) -> i32 {
        //self.grid.iter().flat_map(|row|row).fold::<i32, fn(i32, &E) -> i32>(0, |total, c| (total as i32)  + (*c as i32))
        let grids_on = self.grid.iter().flat_map(|row|row).map::<i32, fn(&E)->i32>(|ii| { 
            let val = ii.clone();
            let vali32 = val.into();
        vali32}).sum();
        grids_on
    }

    fn width(&self) -> usize {
        return self.size;
    }
}
    
fn main() {
    let lines : Vec<String> = read_file("input.txt").unwrap();

    let grid_width = lines[0].len();
    let mut board = Board::<i32>::new(grid_width);

    lines.iter().enumerate().for_each(|(y,l)| {
        l.chars().enumerate().for_each(|(x,c)| {
            match c {
                '.' => board.set(x as i32,y as i32,&0),
                '#' => board.set(x as i32,y as i32,&1),
                _ => panic!("bad char {} at {},{}", c, x, y)
            }
        });
    });

    println!("board: {}", board.print());

    if PART == 2 {
        board.set(0,0,&1);
        board.set(0,grid_width as i32 -1,&1);
        board.set(grid_width as i32-1,0,&1);
        board.set(grid_width as i32-1,grid_width as i32-1,&1);
    }
    let steps : usize = 100;
    for _ in 0..steps {
    let neighborcountboard = board.visit_clone(|x, y, board| {
        let mut num_neighbors = 0; 
        if let Some(val) = board.get(x-1, y-1) {
            if val == 1 { num_neighbors += 1 };
        }
        if let Some(val) = board.get(x, y-1) {
            if val == 1 { num_neighbors += 1 };
        }
        board.get(x+1, y-1);
        if let Some(val) = board.get(x+1, y-1) {
            if val == 1 { num_neighbors += 1 };
        }
        board.get(x-1, y);
        if let Some(val) = board.get(x-1, y) {
            if val == 1 { num_neighbors += 1 };
        }
        board.get(x+1, y);
        if let Some(val) = board.get(x+1, y) {
            if val == 1 { num_neighbors += 1 };
        }
        board.get(x-1, y+1);
        if let Some(val) = board.get(x-1, y+1) {
            if val == 1 { num_neighbors += 1 };
        }
        board.get(x, y+1);
        if let Some(val) = board.get(x, y+1) {
            if val == 1 { num_neighbors += 1 };
        }
        board.get(x+1, y+1);
        if let Some(val) = board.get(x+1, y+1) {
            if val == 1 { num_neighbors += 1 };
        }
        num_neighbors
    });

    let newboard = board.visit_clone(|x,y,board| {
        let on_off = board.get(x,y).unwrap();
        let nc = neighborcountboard.get(x,y).unwrap();
        if on_off == 1 {
            if PART == 2 {
                // if cornr lights are on, they stay on.
                if (x == 0 && y == 0) ||
                   (((x as usize) == board.width()-1) && (y == 0)) ||
                   (((x == 0) && (y as usize == board.width()-1))) ||
                   (((x as usize) == board.width()-1) && ((y as usize) == board.width()-1)) {
                    return 1;
                } else if nc == 2 || nc == 3 {
                    return 1;
                } else {
                    return 0;
                }
            } else {
                if nc == 2 || nc == 3 {
                    return 1;
                } else {
                    return 0;
                }
            }
        } else {
            if nc == 3 {
                return 1;
            } else {
                return 0;
            }
        }
    });
    board = newboard;
    println!("board: {}", board.print());
    }
    let lights_count = board.count_on();
    println!("lights_on : {}", lights_count);
}
