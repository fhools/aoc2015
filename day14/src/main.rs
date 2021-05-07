#[derive(Debug)]
struct Reindeer {
    velocity: i32,
    race_time: i32,
    rest_time_needed: i32,
    cur_distance: i32,
    cur_rest_time: i32,
    cur_race_time: i32,
    state: State, // 1 == racing, 2 == resting
    points: i32
}

#[derive(Debug)]
enum State {
    Resting,
    Racing,
}

const PART : u8 = 2;
fn main() {
    let mut reindeers = vec![
        Reindeer {
            velocity: 19,
            race_time: 7,
            rest_time_needed: 124,
            cur_distance: 0,
            cur_rest_time: 0,
            cur_race_time: 0,
            state: State::Racing,
            points: 0,
        },
        Reindeer {
            velocity: 3,
            race_time: 15,
            rest_time_needed: 28,
            cur_distance: 0,
            cur_rest_time: 0,
            cur_race_time: 0,
            state: State::Racing,
            points: 0,
        },
        Reindeer {
            velocity: 19,
            race_time: 9,
            rest_time_needed: 164,
            cur_distance: 0,
            cur_rest_time: 0,
            cur_race_time: 0,
            state: State::Racing,
            points: 0,
        },
        Reindeer {
            velocity: 19,
            race_time: 9,
            rest_time_needed: 158,
            cur_distance: 0,
            cur_rest_time: 0,
            cur_race_time: 0,
            state: State::Racing,
            points: 0,
        },
        Reindeer {
            velocity: 13,
            race_time: 7,
            rest_time_needed: 82,
            cur_distance: 0,
            cur_rest_time: 0,
            cur_race_time: 0,
            state: State::Racing,
            points: 0,
        },
        Reindeer {
            velocity: 25,
            race_time: 6,
            rest_time_needed: 145,
            cur_distance: 0,
            cur_rest_time: 0,
            cur_race_time: 0,
            state: State::Racing,
            points: 0,
        },
        Reindeer {
            velocity: 14,
            race_time: 3,
            rest_time_needed: 38,
            cur_distance: 0,
            cur_rest_time: 0,
            cur_race_time: 0,
            state: State::Racing,
            points: 0,
        },
        Reindeer {
            velocity: 3,
            race_time: 16,
            rest_time_needed: 37,
            cur_distance: 0,
            cur_rest_time: 0,
            cur_race_time: 0,
            state: State::Racing,
            points: 0,
        },
        Reindeer {
            velocity: 25,
            race_time: 6,
            rest_time_needed: 143,
            cur_distance: 0,
            cur_rest_time: 0,
            cur_race_time: 0,
            state: State::Racing,
            points: 0,
        },
    ];

    let mut seconds = 0;
    loop {
        for r in reindeers.iter_mut() {
            if let State::Resting = r.state {
                r.cur_rest_time += 1;
                if r.cur_rest_time  == r.rest_time_needed {
                    r.cur_rest_time = 0;
                    r.cur_race_time = 0;
                    r.state = State::Racing
                } else {
                   
                }
            } else {
                r.cur_race_time += 1;
                r.cur_distance += r.velocity;
                if r.cur_race_time  == r.race_time {
                    r.state = State::Resting;
                }
            }
        }
        seconds += 1;

        if PART == 2 {
            let leader = reindeers.iter().max_by(|r1, r2| r1.cur_distance.cmp(&r2.cur_distance)).unwrap();
            let lead_distance = leader.cur_distance;
            for r in reindeers.iter_mut() {
                if r.cur_distance == lead_distance {
                    r.points += 1;
                }
            }
        }
        if seconds == 2503 {
            break;
        }
    }

    for r in reindeers.iter() {
        println!("r: {:?}", r);
    }

    let winner : &Reindeer;
    if PART == 1 {
        winner = reindeers.iter().max_by(|r1, r2| r1.cur_distance.cmp(&r2.cur_distance)).unwrap();
        println!("winner: {}", winner.cur_distance);
    } else {
        winner = reindeers.iter().max_by(|r1, r2| r1.points.cmp(&r2.points)).unwrap();
        println!("winner: {}", winner.points);
    }
   
}
