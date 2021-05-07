use std::io::prelude::*;
use std::cmp::max;
const CAPACITY : i32 = 100;
const PART : u8 = 2;
/* 
    input
    Sprinkles: capacity 2, durability 0, flavor -2, texture 0, calories 3
    Butterscotch: capacity 0, durability 5, flavor -3, texture 0, calories 3
    Chocolate: capacity 0, durability 0, flavor 5, texture -1, calories 8
    Candy: capacity 0, durability -1, flavor 0, texture 5, calories 8
*/

fn main() {
    let num_attrs = 4;
    let ingredients = vec![
        vec![2,0,-2,0],
        vec![0,5,-3,0],
        vec![0,0,5,-1],
        vec![0,-1,0,5]
    ];

    let cal = vec![3,3,8,8];
    let mut scores : Vec<i32> = vec![];
    for i in 1..(CAPACITY+1) {
        let c2 = CAPACITY - i;
        for j in 1..(c2+1) {
            let c3 = CAPACITY - i - j;
            for k in 1..(c3+1) {
                let c4 = CAPACITY - i - j - k;
                for m in 1..(c4+1) {
                    let total: i32= vec![i,j,k,m].iter().sum();
                    let calories_total = i*cal[0] + j*cal[1] + k*cal[2] + m*cal[3];
                    if (PART == 2 && calories_total == 500 && total == CAPACITY) ||
                       (PART == 1 && total == CAPACITY) {
                        
                        let recipe = vec![i,j,k,m];
                        //println!("total: {:?}", recipe);
                        let mut ingredients_weighted : Vec<Vec<i32>> = vec![];
                        for z in 0..ingredients.len() {
                            let ingredient_weighted : Vec<i32> = ingredients[z].iter().map(|zz| zz * recipe[z]).collect();
                            //println!("ingredient_weighted: {:?}", ingredient_weighted);
                            ingredients_weighted.push(ingredient_weighted);
                        }

                        let mut sums : Vec<i32> = vec![];
                        for z in 0..num_attrs {
                            // println!("{} {} {} {}",
                            // ingredients_weighted[0][z],
                            // ingredients_weighted[1][z],
                            // ingredients_weighted[2][z],
                            // ingredients_weighted[3][z]));
                            let mut sum : i32 = ingredients_weighted[0][z] + ingredients_weighted[1][z] + ingredients_weighted[2][z] + ingredients_weighted[3][z];
                            sum = max(0, sum);
                            sums.push(sum);
                        }
                        if sums.iter().any(|s| *s == 0)  {
                            continue;
                        } 
                        //println!("sums: {:?}", sums);
                        let p = sums.iter().fold(1, |prod, next| prod * next);
                        scores.push(p);
                        //println!("{:?}", p);
                    }
                }
            }
        }
    }

    let max_score = scores.iter().max().unwrap();
    println!("max score: {}", max_score);
}
