use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

// This does not work, its a dead end, never converges to "e"
fn rewrite(src: &String, goal: &String, rules: &Vec<(String,String)>, steps: i32) -> i32 {

    let mut rulesstack: Vec<(String,i32)> = Vec::new();
    rulesstack.push((src.clone(), steps));
    let mut seenexpansions: HashSet<String> = HashSet::new();
    //let mut collectsteps: Vec<_> = Vec::new();
    loop {
        let cur_src = rulesstack.pop().unwrap();
        println!("cur_src: {:?}", cur_src);
        if cur_src.0 == *goal {
            println!("got it in {} steps", cur_src.1);
            return cur_src.1;
        }
        let goalkeys : Vec<_> = rules.iter().map(|(g,_)| g).collect();
        for (rule,expand) in rules.iter() {
            let m: Vec<_> = cur_src.0.rmatch_indices(rule).collect(); 
            // for mi in m {
            //     let prefix = cur_src.0[0..mi.0].trim().to_string();
            //     let rest = cur_src.0[mi.0+mi.1.len()..].trim().to_string();
            //     let result = format!("{}{}{}",prefix,expand,rest);
            //     //println!("{}: {:?} -> {:?} = {:?}", steps, rule,expand, result);
            //     //println!("{}: {}",steps+1, result);
            //     // let mut found = false;
            //     // for g in goalkeys.iter() {
            //     //     if result.rmatch_indices(*g).collect::<Vec<_>>().len() > 0 {
            //     //         found = true;
            //     //     }
            //     // }
                
            //     //let astep = rewrite(&result, goal, rules, steps+1);
            //     //collectsteps.push(astep);
            //    //} 
            // }
            if m.len() > 0 {
                let result = cur_src.0.replace(rule,expand);
                rulesstack.push((result.clone(), cur_src.1 + (m.len() as i32)));
            }
            // if ! seenexpansions.contains(&result)  {
                
            //     seenexpansions.insert(result.clone());
            // }
        }
    }
    
    //let goals : Vec<String> = rules.iter().map(|(_,e)| e).cloned().collect();
   
    
    // //println!("steps: {:?}", collectsteps);
    // if collectsteps.len() == 0 {
    //     return i64::MAX;
    // }
    // return *collectsteps.iter().min().unwrap();
}

// This is another dead end, tried do do a greedy algorithm, doesn't converge either
fn replace_iter(input: &String, inverserules: &Vec<(String, String)>) -> i32{
    let mut steps : i32= 0;
    let mut cur_str = input.clone();
    while cur_str != "e" {
        for (rule, expand) in inverserules.iter() {
            let cur_str2 = cur_str.clone();
            let m: Vec<_> = cur_str2.rmatch_indices(rule).collect(); 
            for mi in m {
                steps += 1;
                let prefix = cur_str2[0..mi.0].trim().to_string();
                let rest = cur_str2[mi.0+mi.1.len()..].trim().to_string();
                println!("{}->{}", rule,expand);
                let result = format!("{}{}{}",prefix,expand,rest);
                println!("{}", result);
                if result != cur_str && result.len() < cur_str2.len() {
                    cur_str = result;
                }
            }
        }
    }
    return steps; 
}

/*

    // I got this answer from reddit, I'll try to explain my understanding of it. 
    The rules can be summarized as
    e => XX , X=>XX
    These rules will take count(tokens) - 1 steps

    1. X => X "Rn" X "Ar"
    2. X => X "Rn" X "Y" X "Ar"
    3. X => X "RN" X "Y" X "Y" X "Ar"

    Each of 1.) reduces the steps by 1
    Each of 2.), and 3.) reduces the steps by 2 each going from 8 to 6 or 6 to 4.

    Since all the left side productions start with Uppercase,
    and we can count all the upper case characters to get the total "tokens"
    substracting the other rules gives us the total steps.
*/
fn part2_calc(s: &String) -> i32 {
    let num_targets = s.chars().filter(|c| c.is_uppercase()).collect::<Vec<char>>().len();
    let num_rn = s.match_indices("Rn").collect::<Vec<_>>().len();
    let num_ar = s.match_indices("Ar").collect::<Vec<_>>().len();
    let num_y = s.match_indices("Y").collect::<Vec<_>>().len();

    println!("num upper: {}\nnum_rn: {}\nnum_ar: {}\nnum_y: {}", num_targets, num_rn, num_ar, num_y);
    let count = num_targets - num_rn - num_ar - 2*num_y -1;
    return count as i32;

}



fn main() {
    let mut transmap : HashMap<String, Vec<String>> = HashMap::new();

    // transmap.insert("H".to_string(), vec!["HO".to_string(), "OH".to_string()]);
    // transmap.insert("O".to_string(), vec!["HH".to_string()]);
    // transmap.insert("e".to_string(), vec!["O".to_string(), "H".to_string()]);


    transmap.insert("Al".to_string(),vec!["ThF".to_string(), "ThRnFAr".to_string()]); 
    transmap.insert("B".to_string(),vec!["BCa".to_string(), "TiB".to_string(),"TiRnFAr".to_string()]);
    transmap.insert("Ca".to_string(),vec![ "CaCa".to_string(), "PB".to_string(), "PRnFAr".to_string(),
                    "SiRnFYFAr".to_string(),"SiRnMgAr".to_string(), "SiTh".to_string(),]);
    transmap.insert("F".to_string(),vec![ "CaF".to_string(), "PMg".to_string(), "SiAl".to_string(),]);
    transmap.insert("H".to_string(),vec![ "CRnAlAr".to_string(), "CRnFYFYFAr".to_string(), "CRnFYMgAr".to_string(), 
                    "CRnMgYFAr".to_string(),"HCa".to_string(),"NRnFYFAr".to_string(),
                    "NRnMgAr".to_string(), "NTh".to_string(),"OB".to_string(),  "ORnFAr".to_string(),]);
    transmap.insert("Mg".to_string(),vec![ "BF".to_string(),"TiMg".to_string(),]);
    transmap.insert("N".to_string(),vec![ "CRnFAr".to_string(), "HSi".to_string(),]);
    transmap.insert("O".to_string(),vec![ "CRnFYFAr".to_string(),"CRnMgAr".to_string(), "HP".to_string(), "NRnFAr".to_string(),"OTi".to_string(),]);
    transmap.insert("P".to_string(),vec![ "CaP".to_string(),"PTi".to_string(),"SiRnFAr".to_string(),]);
    transmap.insert("Si".to_string(),vec![ "CaSi".to_string(),]);
    transmap.insert("Ti".to_string(),vec![ "BP".to_string(),"TiTi".to_string(),]);
    transmap.insert("e".to_string(), vec!["HF".to_string(),"NAl".to_string(),"OMg".to_string(),]);

    //let input = "e".to_string();
    let mut input = read_to_string("input.txt").unwrap();   
    input.pop(); // remove trailing newline
    println!("input is: {:?}", input);
    let mut calibrationsmap : HashMap<String,bool> = HashMap::new();
    let mut calibrationsset : HashSet<String> = HashSet::new();

    for (k,v) in transmap.iter() {
        for r in v.iter() {
            let m: Vec<_> = input.match_indices(k).collect();
            println!("found {} matches for {}", m.len(), k);
            for mi in m {
                let prefix = input[0..mi.0].trim().to_string();
                let rest = input[mi.0+mi.1.len()..].trim().to_string();
                let result = format!("{}{}{}",prefix,r,rest);
                //println!("{:?} -> {:?} = {:?}", k,r, result);
                calibrationsmap.insert(result.clone(), true);
                calibrationsset.insert(result.clone());
            }
        }
    }

    // println!("count: {}", calibrationsmap.keys().len());
    println!("count: {}", calibrationsset.len());

    // PART 2
    let mut rules : Vec::<(String, String)> = Vec::new();
    for (k,v) in transmap.iter() {
        for r in v.iter() {
            rules.push((k.clone(),r.clone()));
        }
    }

    let mut inverserules : Vec::<(String, String)> = Vec::new();

    for (k,v) in transmap.iter() {
        for r in v.iter() {
            inverserules.push((r.clone(),k.clone()));
        }
    }

// Dead end..
//    let mut inverserules2 = vec![("ThF".to_string(), "Al".to_string()),("ThRnFAr".to_string(), "Al".to_string()),("BCa".to_string(), "B".to_string()),("TiB".to_string(), "B".to_string()),("TiRnFAr".to_string(), "B".to_string()),("CaCa".to_string(), "Ca".to_string()),("PB".to_string(), "Ca".to_string()),("PRnFAr".to_string(), "Ca".to_string()),("SiRnFYFAr".to_string(), "Ca".to_string()),("SiRnMgAr".to_string(), "Ca".to_string()),("SiTh".to_string(), "Ca".to_string()),("CaF".to_string(), "F".to_string()),("PMg".to_string(), "F".to_string()),("SiAl".to_string(), "F".to_string()),("CRnAlAr".to_string(), "H".to_string()),("CRnFYFYFAr".to_string(), "H".to_string()),("CRnFYMgAr".to_string(), "H".to_string()),("CRnMgYFAr".to_string(), "H".to_string()),("HCa".to_string(), "H".to_string()),("NRnFYFAr".to_string(), "H".to_string()),("NRnMgAr".to_string(), "H".to_string()),("NTh".to_string(), "H".to_string()),("OB".to_string(), "H".to_string()),("ORnFAr".to_string(), "H".to_string()),("BF".to_string(), "Mg".to_string()),("TiMg".to_string(), "Mg".to_string()),("CRnFAr".to_string(), "N".to_string()),("HSi".to_string(), "N".to_string()),("CRnFYFAr".to_string(), "O".to_string()),("CRnMgAr".to_string(), "O".to_string()),("HP".to_string(), "O".to_string()),("NRnFAr".to_string(), "O".to_string()),("OTi".to_string(), "O".to_string()),("CaP".to_string(), "P".to_string()),("PTi".to_string(), "P".to_string()),("SiRnFAr".to_string(), "P".to_string()),("CaSi".to_string(), "Si".to_string()),("ThCa".to_string(), "Th".to_string()),("BP".to_string(), "Ti".to_string()),("TiTi".to_string(), "Ti".to_string()),("HF".to_string(), "e".to_string()),("NAl".to_string(), "e".to_string()),("OMg".to_string(), "e".to_string())];
//    inverserules2.sort_by(|a,b| b.0.len().cmp(&a.0.len()));
//     println!("inverserules: {:?}", inverserules2);
//     let steps = rewrite(&input, &"e".to_string(),&inverserules2, 0);
//     //let steps = replace_iter(&input, &inverserules2);
//     println!("steps: {}", steps);


    println!("part 2 steps: {}", part2_calc(&input));

}
