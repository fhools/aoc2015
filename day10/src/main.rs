fn look_and_say(s: &String) -> String {
    let mut t = String::new();
    let mut prev_c = s.chars().next().unwrap();
    let mut cur_c_cnt = 1;
    for c in s.chars().skip(1) {
        if prev_c == c {
            cur_c_cnt +=1;
        } else {
            t.extend(format!("{}{}", cur_c_cnt, prev_c).chars());
            prev_c = c;
            cur_c_cnt = 1;
        }
    }

    t.extend(format!("{}{}",cur_c_cnt, prev_c).chars());
    t
}

fn main() {
    let mut s = "3113322113".to_string();
    //println!("{}",s);
    for i in 0..50 {
        s = look_and_say(&s);
        //println!("{}", s);
    }
    println!("40th iteration is {} chars long", s.len())
}
