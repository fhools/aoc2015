/* 
Text from website
--- Day 17: No Such Thing as Too Much ---

The elves bought too much eggnog again - 150 liters this time. To fit it all
into your refrigerator, you'll need to move it into smaller containers. You take
an inventory of the capacities of the available containers.

For example, suppose you have containers of size 20, 15, 10, 5, and 5 liters. If
you need to store 25 liters, there are four ways to do it:

15 and 10
20 and 5 (the first 5)
20 and 5 (the second 5)
15, 5, and 5
Filling all containers entirely, how many different combinations of containers
can exactly fit all 150 liters of eggnog?


*/
fn count_solutions_find_min(v: &Vec<i32>, k: usize, cur_path: Vec<bool>, total: &mut i32, min_num: &mut i32) {
    if k == v.len() {
        let sum : i32 = cur_path.iter().enumerate().map(|(i, x)| match x { true => v[i], false => 0 }).sum();
        if sum == 150 {
            let num_containers  : i32 = cur_path.iter().map(|x| match x { true => 1, false => 0}).sum();
            if num_containers < *min_num {
                *min_num = num_containers;
            }
            *total += 1;
            
        }
        return;
    }
    let mut nextpath1 = cur_path.clone();
    let mut nextpath2 = cur_path.clone();
    nextpath1.push(true);
    nextpath2.push(false);
    count_solutions_find_min(v, k+1, nextpath1, total, min_num);
    count_solutions_find_min(v, k+1, nextpath2, total, min_num);
}

fn count_solutions_with_min_container(v: &Vec<i32>, k: usize, cur_path: Vec<bool>, total: &mut i32, min_num: &i32) {
    if k == v.len() {
        let sum : i32 = cur_path.iter().enumerate().map(|(i, x)| match x { true => v[i], false => 0 }).sum();
        let num_containers  : i32 = cur_path.iter().map(|x| match x { true => 1, false => 0}).sum();
        if sum == 150 && *min_num == num_containers {
            *total += 1;
        }
        return;
    }
    let mut nextpath1 = cur_path.clone();
    let mut nextpath2 = cur_path.clone();
    nextpath1.push(true);
    nextpath2.push(false);
    count_solutions_with_min_container(v, k+1, nextpath1, total, min_num);
    count_solutions_with_min_container(v, k+1, nextpath2, total, min_num);
}

fn main() {
    let container_sizes: Vec<i32> = vec![
        50, 44, 11, 49, 42, 46, 18, 32, 26, 40, 21, 7, 18, 43, 10, 47, 36, 24, 22, 40,
    ];

    let mut total = 0;
    let mut min_num = i32::MAX;
    count_solutions_find_min(&container_sizes, 0, vec![], &mut total, &mut min_num);
    println!("ways to get 150 liters: {}, minimum: {}", total, min_num);
    // reset
    total = 0;
    count_solutions_with_min_container(&container_sizes, 0, vec![], &mut total, &min_num);

    println!("ways to get 150 liters: {} with minimum: {}", total, min_num);
}
