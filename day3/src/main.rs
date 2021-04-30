use std::fs::{read_to_string};
use std::collections::HashMap;
fn main() {
    let input = read_to_string("input.txt");
    if let Ok(input) = input {
        let mut coords = (0,0);
        let mut house_gift_count = HashMap::new();
        house_gift_count.insert(coords, 1);
        for i in input.chars() {
            match i {
                '^' => {
                    coords.1 += 1;
                },
                '<' => {
                    coords.0 -= 1;
                },
                '>' => {
                    coords.0 += 1;
                },
                'v' => {
                    coords.1 -= 1;
                }
                _ => {
                    continue;
                }
            }
            house_gift_count.entry(coords)
                .and_modify(|e| { *e += 1 })
                .or_insert(1);
        }

        let houses_multiple_gift = house_gift_count.values()
            .map::<i32, fn(&i32)->i32>(|v| {
            if *v >= 1 {1}  else {0}
        }).fold(0, |sum, x| sum + x);
        println!("houses with at least one gifts: {}", houses_multiple_gift);
    } else if let Err(err) =  input {
        println!("Could not read input: {}", err);
    }

   

}
