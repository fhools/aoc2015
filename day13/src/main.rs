use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

fn perms(v: &Vec<i32>) -> Vec<Vec<i32>> {
    if v.len() == 1 {
        return vec![v.clone()];
    }

    let mut perm: Vec<Vec<i32>> = vec![];
    for e in v.iter() {
        let rest = v.iter().filter(|i| **i != *e).cloned().collect();
        let subperm = perms(&rest);

        for s in subperm.iter() {
            let mut subperm_extended = s.clone();
            subperm_extended.push(*e);
            perm.push(subperm_extended);
        }
    }
    return perm;
}


fn compute_happiness(v: &Vec<i32>, happymap: &HashMap<(i32,i32), i32>) -> i32 {
    let mut sum = 0;
    for i in 0..v.len() {
        if i == v.len() - 1 {
            let h1 = happymap.get(&(v[i], v[0])).unwrap();
            let h2 = happymap.get(&(v[0], v[i])).unwrap();
            sum += h1 + h2;
        } else {
            let h1 = happymap.get(&(v[i], v[i+1])).unwrap();
            let h2 = happymap.get(&(v[i+1], v[i])).unwrap();
            sum += h1 + h2;
        }
    }
    sum
}

fn vec2name(v: &Vec<i32>, idmap: &HashMap<String, i32>) -> Vec<String> {
    let mut names = vec![];
    v.iter().for_each(|id| {
        for (k,v) in idmap.iter() {
            if *v == *id {
                names.push(k.to_string());
                break;
            }
        }
    });
    names
}

fn main() {
    let fh = File::open("input2.txt").unwrap();
    let reader = BufReader::new(fh);
    let lines : Vec<String> = reader.lines().collect::<Result<_,_>>().unwrap();
    let mut happymap = HashMap::new();
    let mut idmap = HashMap::new();

    let mut id_gen = 0;

    lines.iter().for_each(|l| {
        let mut ll = l.clone();
        ll.pop();
        let toks : Vec<&str> = ll.split(" ").collect();
        let src = toks[0].trim().to_string();
        let dest = toks[10].trim().to_string();
        if !idmap.contains_key(&src) {
            idmap.insert(src.clone(), id_gen);
            id_gen +=1;
        }

        if !idmap.contains_key(&dest) {
            idmap.insert(dest.clone(), id_gen);
            id_gen += 1;
        }
    });

    lines.iter().for_each(|l| {
        let mut ll = l.clone();
        ll.pop();
        let toks : Vec<&str> = ll.split(" ").collect();
        let src = toks[0].trim().to_string();
        let dest = toks[10].trim().to_string();
        let mut cost = toks[3].trim().to_string().parse::<i32>().unwrap();
        
        let gainorlose = toks[2].trim().to_string();
        if gainorlose == "lose" {
            cost = -cost;
        }
        let srcid = *idmap.get(&src).unwrap();
        let destid = *idmap.get(&dest).unwrap();
        happymap.insert((srcid,destid), cost);
    });

    let mut happiness = i32::MIN;
    let mut order = vec![];
    let n = id_gen;
    let v = (0..n).collect::<Vec<i32>>();
    let p = perms(&v);
    
    for i in p.iter() {
        let h = compute_happiness(i, &happymap);

        if h > happiness {
            happiness = h;
            order = i.clone();
        }
    }

    println!("happiness: {}", happiness);
    println!("order: {:?} {:?}", order, vec2name(&order, &idmap));
}
