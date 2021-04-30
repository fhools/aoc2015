use std::fs::{read_to_string};


fn compute_floor(s: &String) -> Result<i32, String> {
    let floor : i32;

    floor = s.chars().map::<i32, fn(char)->i32>(|c| if c == '(' {1} else if c == ')' {-1} else {0}).fold(0,|total, next| total + next);
    Ok(floor)
}
fn main() {
    let inputresult = read_to_string("input.txt");
    if let Ok(input) = inputresult {
        println!("read {} characters", input.len());
        let floor = compute_floor(&input);
        if let Ok(floor_val) = floor {
            println!("floor: {}", floor_val);
        } else if let Err(error) = floor {
            println!("compute_floor err: {}", error);
        }
    }  else if let Err(err) = inputresult {
        println!("error reading file: {}", err);
    }
}
