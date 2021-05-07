fn diag_start(d: i32) -> i32 {
    let mut n = 1;
    for i in 0..d {
        n += i;
    }   
    n
}

fn get_gen_code(code_num:i32) -> u64 {
    let mut codes : Vec<u64> = vec![0;code_num as usize];
    codes[0] = 20151125;
    for i in 1..code_num {
        codes[i as usize] = (codes[i as usize -1] * 252533) % 33554393;
    }
    return codes[code_num as usize-1];
}


fn main() {
    // The grid size needs to be a lot larger 
    // since are only computing the upper left triangle of the grid, not the 
    // lower right triangle of the grid. Even though requested row, col is 
    // ~3000,2900, we'll come up 10000 diagonals.
    const  GRIDSZ : usize = 10000;
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for _ in 0..GRIDSZ {
        grid.push(vec![0; GRIDSZ]);
    }
    // Compute a grid of code numbering i.e 
    // 1 3 6 ...
    // 2 5 ...
    // 4 ...
    // We then extract code number from requested row,col
    // and then generate the nth code.
    grid[0][0] = 1;
    for i in 0..GRIDSZ {
        let diag = i+1;
        let diag_start = diag_start(diag as i32);
        //println!("{}={}", diag, e_start_at_diag(diag));
        let mut x=0;
        let mut y=diag-1;
        
        for j in diag_start..(diag_start+ (diag as i32)  ) {
            //println!("{},{}",x, y);
            grid[y as usize][x as usize] = j;
            x += 1;
            y -= 1;
        }
    }
   
    let x = 3074;
    let y = 2980;
    let code_num =  grid[y as usize][x as usize];
    println!("{},{} = code# {} is code = {}",x,y, code_num, get_gen_code(code_num));
}
