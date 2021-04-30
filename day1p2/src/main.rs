use std::fs::{read_to_string};


fn compute_floor(s: &String) -> Result<Vec<i32>, String> {
    let steps: Vec<i32>;

    steps = s.chars().map::<i32, fn(char)->i32>
        (|c| if c == '(' {1} else if c == ')' {-1} else {0}).collect();
    Ok(steps)
}

fn main() {
    let inputresult = read_to_string("input.txt");
    if let Ok(input) = inputresult {
        println!("read {} characters", input.len());
        let steps = compute_floor(&input);
        if let Ok(steps) = steps {
            let mut cur_floor = 0;
           for (i, step) in steps.iter().enumerate() {
                cur_floor += step;
                if cur_floor == -1 {
                    println!("Basement floor = {}", i+1);
                    return;
                }
           }
           println!("Never reached basement");
        } else if let Err(error) = steps {
            println!("compute_floor err: {}", error);
        }
    }  else if let Err(err) = inputresult {
        println!("error reading file: {}", err);
    }
}
