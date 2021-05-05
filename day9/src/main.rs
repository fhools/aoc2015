use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

fn read_file(s: &str) -> Result<Vec<String>, io::Error> {
    let fh = File::open(s);
    if let Ok(fh) = fh {
        let reader = BufReader::new(fh);
        return Ok(reader.lines().collect::<Result<_,_>>().unwrap());
    }  else if let Err(err) = fh {
        return Err(io::Error::new(err.kind(),"read_file fail"));
    } else {
        return Err(io::Error::new(io::ErrorKind::Other,"read_file fail"));
    }
}

const PART : u8 = 2;
#[derive(Debug, Eq, PartialEq, Clone)]
struct Edge {
    v: i32,
    u: i32,
    cost: i32
}


fn travel_cost(start: i32, edges: &Vec<Vec<Edge>>, unvisited_nodes: &Vec<i32>, total_distance: i32, cmp_op: fn(i32,i32) -> bool) -> Option<i32> {
    if unvisited_nodes.len() == 0 {
        return Some(total_distance)
    }
    let mut min_max_cost : Option<i32> = None;
    for e in &edges[start as usize] {
        if unvisited_nodes.contains(&e.u)  {
            let mut edges2 = edges.clone();
            edges2[start as usize] = edges[start as usize].iter().filter(|e2| **e2!=*e).cloned().collect();
            let new_unvisited_nodes = unvisited_nodes.iter().filter(|w| **w != e.u).cloned().collect();
            let cost = travel_cost(e.u, &edges2 , &new_unvisited_nodes , total_distance + e.cost, cmp_op);
            if let Some(new_cost) = cost {
                if None == min_max_cost {
                    min_max_cost = Some(new_cost)
                } else if let Some(old_min_max_cost) = min_max_cost {
                    if cmp_op(new_cost,old_min_max_cost) {
                        min_max_cost = Some(new_cost);
                    }
                }
            }
        }
    }

    return min_max_cost;
} 

fn main() {
    let mut node_id_gen = 0;
    let mut name2node = HashMap::new();
    let mut nodes : Vec<i32>= vec![];
    let lines = read_file("input.txt").unwrap();

    // First pass through input file, generate string to node id
    lines.iter().for_each(|l| {
        let toks  : Vec<&str> = l.split(' ').collect();
        let v_name = toks[0].trim().to_string();
        let u_name = toks[2].trim().to_string();
       
        if !name2node.contains_key(&v_name) {
            name2node.insert(v_name.clone(), node_id_gen);
            nodes.push(node_id_gen);
            node_id_gen += 1;
        }

        if !name2node.contains_key(&u_name) {
            name2node.insert(u_name.clone(), node_id_gen);
            nodes.push(node_id_gen);
            node_id_gen += 1;
        }
    });
   
    // Allocate vector of edges now that we know the max id
    let mut edges : Vec<Vec<Edge>> = vec![vec![]; node_id_gen as usize];


    lines.iter().for_each(|l| {
        let toks : Vec<&str> = l.split(" ").collect();
        let v_name = toks[0].trim().to_string();
        let u_name = toks[2].trim().to_string();

        let cost = toks[4].trim().to_string().parse::<i32>().unwrap();
        edges[*name2node.get(&v_name).unwrap() as usize].push(Edge{v: *name2node.get(&v_name).unwrap(),
            u: *name2node.get(&u_name).unwrap(),
            cost: cost });
        edges[*name2node.get(&u_name).unwrap() as usize].push(Edge{u: *name2node.get(&v_name).unwrap(),
            v: *name2node.get(&u_name).unwrap(),
            cost: cost });
    });

    let mut min_max_cost : Option<i32> = None;


    let cmp_op : fn(i32,i32) -> bool;
    if PART == 1 {
        cmp_op = |new, curr| {
            new < curr
        }
    } else {
        cmp_op = |new, curr| {
            new > curr
        }
    }
    for i in &nodes {
        let new_edges = nodes.iter().filter(|u| **u != *i).cloned().collect();
        let cost = travel_cost(*i, &edges, &new_edges, 0, cmp_op);
        if let Some(new_cost) = cost {
            if None == min_max_cost {
                min_max_cost = Some(new_cost)
            } else if let Some(old_min_cost) = min_max_cost {
                if cmp_op(new_cost,old_min_cost) {
                    min_max_cost = Some(new_cost);
                }
            }
        }
    }
    if let Some(cost) = min_max_cost {
        println!("cost: {}", cost);
    } else {
        println!("no route");
    }
}
