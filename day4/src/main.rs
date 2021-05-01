use md5::compute;
fn check_hash_5_zeroes(hash: [u8;16]) -> bool {
    for i in 0..3 {
        if i < 2 {
            if hash[i] != 0 {
                return false;
            }
        } else if i == 2 {
            if (hash[i] & 0xF0) != 0  {
                return false
            }
        }
    }
    return true
}

fn check_hash_6_zeroes(hash: [u8;16]) -> bool {
    for i in 0..3 {
            if hash[i] != 0 {
                return false;
            }
    }
    return true
}

// set part = 1 for day 1, = 2 for day 2 puzzle
let part = 2;
fn main() {
    let mut i = 1;
    let key = "bgvyzdsv";
    loop {
        let data = format!("{}{}",key,i.to_string());
        let md5hash = compute(data.as_bytes());
        println!("md5sum({:?}) =  {:?}", data, md5hash);
        if part == 1 {
            if  check_hash_5_zeroes(md5hash.into()) {
                println!("part 1 hit = {}", i);
                break;
            }
        } else {
            if  check_hash_6_zeroes(md5hash.into()) {
                println!("part 2 hit = {}", i);
                break;
            }
        }
       
        i += 1
    }
   
}
