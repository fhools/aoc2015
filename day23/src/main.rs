use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io;

fn read_file(filepath: &str) -> Result<Vec<String>, io::Error> {
    let fh = File::open(filepath)?;
    let reader = BufReader::new(fh);
    let lines : Vec<String> = reader.lines().collect::<Result<_,_>>().unwrap();
    Ok(lines)
}

#[derive(Debug)]
enum Instr {
    Tpl(usize),
    Inc(usize),
    Hlf(usize),
    Jmp(i32),
    Jie(usize, i32),
    Jio(usize, i32),
}


fn main() {
    let lines = read_file("input.txt").unwrap();
    let mut instrs: Vec<Instr> = vec![];

    for l in &lines {
        let toks : Vec<&str> = l.split(" ").collect();
        match toks[0] {
            "tpl" => {
                let reg;
                if toks[1] == "a" {
                    reg = 0;
                } else {
                    reg = 1;
                }
                instrs.push(Instr::Tpl(reg));
            },
            "inc" => {
                let reg;
                if toks[1] == "a" {
                    reg = 0;
                } else {
                    reg = 1;
                }
                instrs.push(Instr::Inc(reg));
            },
            "hlf" => {
                let reg;
                if toks[1] == "a" {
                    reg = 0;
                } else {
                    reg = 1;
                }
                instrs.push(Instr::Hlf(reg));
            },
            "jmp" => {
                let mut offset;
                offset = toks[1][1..].to_string().parse::<i32>().unwrap();
                if toks[1][0..1] == "-".to_string() {
                    offset = -offset;
                }
                instrs.push(Instr::Jmp(offset));
            },
            "jie" => {
                let reg;
                if toks[1][0..1].to_string() == "a" {
                    reg = 0;
                } else {
                    reg = 1;
                }
                let mut offset;
                offset = toks[2][1..].to_string().parse::<i32>().unwrap();
                if toks[2][0..1] == "-".to_string() {
                    offset = -offset;
                }
                instrs.push(Instr::Jie(reg, offset));
            },
            "jio" => {
                let reg;
                if toks[1][0..1].to_string() == "a" {
                    reg = 0;
                } else {
                    reg = 1;
                }
                let mut offset;
                offset = toks[2][1..].to_string().parse::<i32>().unwrap();
                if toks[2][0..1] == "-".to_string() {
                    offset = -offset;
                }
                instrs.push(Instr::Jio(reg, offset));
            },
            _ => {}
        }
    }

    let mut pc : i32 = 0;
    let mut regs: [u32;2] = [0,0];
    const PART : u8 = 2;
    if PART == 1{
        regs[0] = 1;
    }
    for (i, instr) in instrs.iter().enumerate() {
        println!("{}: {:?}", i , instr);
    }
    loop {
        println!("Before: {:?} Executing: {}: {:?}", regs, pc, instrs[pc as usize]);
        match instrs[pc as usize] {
            Instr::Tpl(reg) => {
                regs[reg] = regs[reg]*3;
                pc += 1;
            },
            Instr::Hlf(reg) => {
                regs[reg] = regs[reg]/2;
                pc += 1;
            },
            Instr::Jmp(offset) => {
                pc += offset;
            },
            Instr::Inc(reg) => {
                regs[reg] +=1;
                pc += 1;
            },
            Instr::Jie(reg,offset) => {
                if (regs[reg] %2) == 0 {
                    pc += offset;
                } else {
                    pc += 1;
                }
            },
            Instr::Jio(reg,offset) => {
                if regs[reg] == 1 {
                    pc += offset;
                } else {
                    pc += 1;
                }
            }
        }
        println!("After: {:?}", regs);
        if pc as usize >= instrs.len() {
            println!("pc {} is out of range. execution finished", pc);
            break;
        }
    }
    println!("regs: {:?}", regs);
}
