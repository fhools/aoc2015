fn incr(num: &Vec<u32>) -> Vec<u32> {
    // d is the digit in reverse order, lsb to msb
    const BASE : u32 = 26;
    let mut c = 1;
    let mut d = num.clone();
    for i in 0..d.len() {
        let dx = d[i] + c;
        d[i] = dx;
        if dx == BASE {
            c = 1;
            d[i] = 0;
        } else {
            c = 0;
        }
    }    
    return d;
}

fn vec2str(v: &Vec<u32>) -> String {
    let mut s = "".to_string();
    v.iter().rev().for_each(|x| {
        s.push((*x as u8 + ('a' as u8)) as char);
    });
    s
}

fn rule1(v: &Vec<u32>) -> bool {
    for i in 0..v.len()-2 {
        if v[i] == (v[i+1]+1) && 
            v[i+1] == (v[i+2]+1) {
                return true;
            }
    }
    return false;
}

fn rule2(v: &Vec<u32>) -> bool {
    for x in v.iter() {
        let c = ((*x as u8) + ('a' as u8)) as char;
        match c {
            'o' |  'l' |  'i' => {
                return false;
            }, 
            _ => {}
        }
    }
    return true
}

fn rule3(v: &Vec<u32>) -> bool {
    let mut pi = Vec::new();
    for i in 0..v.len()-1 {
        if v[i]==v[i+1] {
            pi.push(i);
        }
    }

    if pi.len() > 2 {
        return true;
    } else if pi.len() == 2  && (pi[0] + 1) != pi[1] {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let s = "cqjxjnds".to_string();
    let mut vx : Vec<u32> = s.chars().map::<u32, fn(char)->u32>(|c| (c as u32) - ('a' as u32)).rev().collect();
    loop {
        vx = incr(&vx);
        if rule1(&vx) && rule2(&vx) && rule3(&vx) {
            break;
        }
    }
    let mut newpasswd = vec2str(&vx);
    println!("{}", newpasswd);
    vx =  newpasswd.chars().map::<u32, fn(char)->u32>(|c| (c as u32) - ('a' as u32)).rev().collect();
    loop {
        vx = incr(&vx);
        if rule1(&vx) && rule2(&vx) && rule3(&vx) {
            break;
        }
    }
    newpasswd = vec2str(&vx);
    println!("{}", newpasswd);

    
}
