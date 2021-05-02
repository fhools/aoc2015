use std::fs::File;
use std::io;
use std::io::{BufReader};
use std::io::prelude::*;
use std::collections::HashMap;

/* Approach
   Use topogical sort. Kahn's algorithm from wikipedia

   L ← Empty list that will contain the sorted elements
    S ← Set of all nodes with no incoming edge

    while S is not empty do
        remove a node n from S
        add n to L
        for each node m with an edge e from n to m do
            remove edge e from the graph
            if m has no other incoming edges then
                insert m into S

    if graph has edges then
        return error   (graph has at least one cycle)
    else 
        return L   (a topologically sorted order)
    
    Sort the lines, set starting signals
    Execute each line, keeping track of node's output.
    Extract the specified signal at the end of execution
*/

#[derive(Debug)]
enum ParseError {
    Err(String)
}

#[derive(Debug,  Clone, Eq, PartialEq, Hash)]
enum Operand {
    Node(String),
    Value(u16)
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Instruction {
    And{ i1: Operand, i2: Operand, o: String},
    Or{ i1: Operand, i2: Operand, o: String},
    Not{ i: String, o: String},
    Lshift{i: String, bits: u16, o: String},
    Rshift{i: String, bits: u16, o: String},
    Assign{src: Operand, o: String}
}

struct InstructionWithExecute {
    instr: Instruction,
    executed: bool
}

fn read_file(s: &str) -> Result<Vec<String>, io::Error> {
    let fh = File::open(s);
    if let Ok(fh) = fh {
        let reader = BufReader::new(fh);
        return Ok(reader.lines().collect::<Result<_,_>>().unwrap());
    }  else if let Err(err) = fh {
        return Err(err);
    } else {
        return Err(io::Error::new(io::ErrorKind::Other,"read_file fail"));
    }
}

fn parse_operand(s: &str) -> Result<Operand, ParseError> {
    if let Ok(i) = s.parse::<u16>() {
        return Ok(Operand::Value(i));
    } else {
        return Ok(Operand::Node(s.trim().to_string()));
    }
}

fn parse_line(s: &str) -> Result<Instruction, ParseError> {

    let tok : Vec<&str> = s.split("->").collect();
    let o = tok[1].trim();
    let expr = tok[0];

    let tok2 : Vec<&str> = expr.split(' ').collect();
    if tok2[0] == "NOT" {
        let operand = tok2[1];
        return Ok(Instruction::Not{i: operand.to_string(), o: o.to_string()});     
    } else {
        if tok2[1] == "AND" {
            return Ok(Instruction::And{
                i1: parse_operand(tok2[0]).unwrap(), 
                i2: parse_operand(tok2[2]).unwrap(),
                o: o.trim().to_string()});
        } else if tok2[1] == "OR" {
            return Ok(Instruction::Or{
                i1: parse_operand(tok2[0]).unwrap(), 
                i2: parse_operand(tok2[2]).unwrap(),
                o: o.trim().to_string()});
        } else if tok2[1] == "RSHIFT" {
            return Ok(Instruction::Rshift{
                i: tok2[0].trim().to_string(),
                bits: tok2[2].parse::<u16>().unwrap(),
                o: o.trim().to_string()
            })
        } else if tok2[1] == "LSHIFT" {
            return Ok(Instruction::Lshift{
                i: tok2[0].trim().to_string(),
                bits: tok2[2].parse::<u16>().unwrap(),
                o: o.trim().to_string()
            })
        } else {
            println!("is assign??? {}", s);
            return Ok(Instruction::Assign{
                src: parse_operand(tok2[0]).unwrap(),
                o: o.trim().to_string()
            })
        }
    }
}

fn initialize(mem: &mut HashMap<String,u16>, instrs: &Vec<Instruction>) -> Vec<String> {
    let mut next_inputs : Vec<String> = vec![]; 
    instrs.iter().for_each(|i| 
        match i {
            Instruction::Assign{ src: Operand::Value(val), o: output} => {
                println!("Setting {} to {}", output, val);
                mem.insert(output.to_string(), *val);
                next_inputs.push(output.to_string());
            },
            _ => {}
        });
    return next_inputs
}

fn exec(mem: &mut HashMap<String,u16>, exec_status: &mut HashMap<Instruction, bool>, instrs: &Vec<Instruction>) -> Vec<String> {
    let mut outputs: Vec<String> = vec![];
    
    instrs.iter().for_each(|i|  {
        // if already executed
        if exec_status.contains_key(i) {
            //println!("already contain {:?}", i);
            return;
        }
        match i {
            Instruction::Assign{o, src} => {
                if let Operand::Node(src) = src {
                    if mem.contains_key(src) {
                        mem.insert(o.to_string(), *mem.get(src).unwrap());
                        exec_status.insert(i.clone(), true);
                        println!("execing {:?} {} = {:X}", i, o, *mem.get(src).unwrap());
                        outputs.push(o.to_string());
                    }
                } else if let Operand::Value(v) = src {
                    mem.insert(o.to_string(), *v);
                    exec_status.insert(i.clone(), true);
                    println!("execing {:?} {} = {:X}", i, o , v);
                    outputs.push(o.to_string());
                }
                
            },
            
            Instruction::And{o, i1, i2 } => {
                let mut x = 0;
                let mut y = 0;
                if let Operand::Value(v) = i1 {
                    x = *v;
                } else if let Operand::Node(v) = i1 {
                    if mem.contains_key(v) {
                        x = *mem.get(v).unwrap();
                    } else {
                        return;
                    }
                }
                if let Operand::Value(v) = i2 {
                    y = *v;
                } else if let Operand::Node(v) = i2 {
                    if mem.contains_key(v) {
                        y = *mem.get(v).unwrap();
                    } else {
                        return;
                    }
                }
                mem.insert(o.to_string(), x & y);
                exec_status.insert(i.clone(), true);
                println!("execing {:?} {:X} & {:X} = {:X}", i, x ,y , x & y);
                outputs.push(o.to_string());
            },

            Instruction::Or{o, i1, i2 } => {
                let mut x = 0;
                let mut y = 0;
                if let Operand::Value(v) = i1 {
                    x = *v;
                } else if let Operand::Node(v) = i1 {
                    if mem.contains_key(v) {
                        x = *mem.get(v).unwrap();
                    } else {
                        return;
                    }
                }
                if let Operand::Value(v) = i2 {
                    y = *v;
                } else if let Operand::Node(v) = i2 {
                    if mem.contains_key(v) {
                        y = *mem.get(v).unwrap();
                    } else {
                        return;
                    }
                }
                mem.insert(o.to_string(), x | y);
                exec_status.insert(i.clone(), true);
                println!("execing {:?} {:X} | {:X} = {:X}", i, x ,y , x | y);
                outputs.push(o.to_string());
            },
            Instruction::Not{o, i:src} => {
                if mem.contains_key(src) {
                    mem.insert(o.to_string(), ! *mem.get(src).unwrap());
                    exec_status.insert(i.clone(), true);
                    println!("execing {:?} not {:X} = {:X}", i, *mem.get(src).unwrap(), ! *mem.get(src).unwrap() );
                    outputs.push(o.to_string());
                }
            },
            Instruction::Lshift{o, i: inp, bits} => {
                if mem.contains_key(inp) {
                    let val = *mem.get(inp).unwrap();

                    mem.insert(o.to_string(),val << *bits );
                    exec_status.insert(i.clone(), true);
                    println!("execing {:?} {:X} << {} = {:X}", i, val, *bits, val << *bits);
                    outputs.push(o.to_string());
                }
               
            },
            Instruction::Rshift{o, i: inp, bits} => {
                if mem.contains_key(inp) {
                    let val = *mem.get(inp).unwrap();
                    mem.insert(o.to_string(),val >> *bits );
                    exec_status.insert(i.clone(), true);
                    println!("execing {:?} {:X} >> {} = {:X}", i, val, *bits, val >> *bits);
                    outputs.push(o.to_string());
                }
                
            }
            _ => {
                    println!("unknown instr {:?}", i);
            }
        }
        
    });

    return outputs;
}

fn has_inputs(mem: &HashMap<String,u16>, instr: &Instruction, s: &Vec<String>) -> bool {
    match instr {
        Instruction::And{ i1, i2, ..} => {
            if let Operand::Node(n) = i1 {
                if s.contains(n) {
                    return mem.contains_key(n);
                }
            } else if let Operand::Node(n) = i2 {
                if s.contains(n) {
                    return mem.contains_key(n);
                }
            }
            return false;
        },
        Instruction::Or{ i1, i2, ..} => {
            if let Operand::Node(n) = i1 {
                if s.contains(n) {
                    return mem.contains_key(n);
                }
            } else if let Operand::Node(n) = i2 {
                if s.contains(n) {
                    return mem.contains_key(n);
                }
            }
            return false;
        },
        Instruction::Not{ i: n,.. } => {
            if s.contains(n) {
                return mem.contains_key(n);
            }
            return false;
        },
        Instruction::Lshift{ i,..} => {
            if s.contains(i) {
                return mem.contains_key(i);
            }
            return false;
        },
        Instruction::Rshift{i, ..} => {
            if s.contains(i) {
                return mem.contains_key(i);
            }
            return false
        },
        Instruction::Assign{src, ..} => {
            if let Operand::Node(n) = src {
                if s.contains(n) {
                    return mem.contains_key(n);
                }
            }
            return false;
        }
    }
}

fn main() -> io::Result<()>{
    let lines = read_file("input.txt").unwrap();
    let mut instr : Vec<Instruction> = lines.iter().map(|l| parse_line(l).unwrap()).collect();
    let mut mem = HashMap::new();
    //let mut instr_exec: Vec<InstructionWithExecute> = vec![];
    let mut exec_status : HashMap<Instruction, bool> = HashMap::new();
    // Initialize execution status
    instr.iter().for_each(|i| {
        println!("{:?}", i);
        //instr_exec.push(InstructionWithExecute{ instr: i.clone(), executed: false});
        //exec_status.insert(i.clone(), false);
    });

    // let mut next_inputs = initialize(&mut mem, &instr);
    // let mut instrs_to_exec : Vec<Instruction>;
    // loop {
    //     instrs_to_exec = instr.iter().filter(|i| has_inputs(&mem, i, &next_inputs)).cloned().collect();
    //     if instrs_to_exec.len() == 0 {
    //         println!("no instructions to exec");
    //         break;
    //     }
    //     next_inputs = exec(&mut mem, &mut exec_status, &instrs_to_exec);
    //     println!("next_inputs: {:?}", next_inputs);
    // }
   
    // We are gonna do something so horrible.
    // Execute all instructions n times. So this runs in n^2
   for _ in 0..instr.len() {
   //    for _ in 0..5{
        exec(&mut mem, &mut exec_status, &instr);
        //println!("keys: {:?}\n\n", exec_status.keys());
    }
    //println!("mem: {:?}", mem);
    println!(" value of wire a is {}", mem.get("a").unwrap());
    Ok(())
}

