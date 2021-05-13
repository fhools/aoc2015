

fn main() {
    const PART : u8 = 2;

    const MAX_HOUSES : usize = 1_000_000;
    const NUM_ELVES : usize = 1_000_000;

    let mut houses : Vec<i32> = vec![0; MAX_HOUSES+1];

    if PART == 1 {
        for i in 1..=NUM_ELVES {
            let mut j = i;
            while j <= MAX_HOUSES {
                houses[j] += 10*i as i32;
                j += i;
            }
        }
    } else {
        for i in 1..=NUM_ELVES {
            let mut j =i;
            let mut house_visit_count = 0;
            while house_visit_count < 50 && j <= MAX_HOUSES {
                houses[j] += 11*i as i32;
                house_visit_count += 1;
                j += i;
            }
        }
    }

    const GOAL: i32 = 36_000_000;
    for (i,h) in houses.iter().enumerate() {
        if *h >= GOAL {
            println!("house # {} has goal", i);
            break;
        }
    }
}
