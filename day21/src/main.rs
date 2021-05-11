/*
    Boss Attributes:
    
        Hit Points: 103
        Damage: 9
        Armor: 2

    Your attributes:
        Hit Points: 100
    
    Ideas:
        Need to outlive the boss, that means doing 104+ damage 
        before he does 100.
        Max armor/range  is 8, which is less than bosses dmg so it will do 
        (dmg - armor) per tick.
        
        Calculate all the armor/ring combos and find cost/per dmg  taken where 
        dmg is (boss_dmg - your_armor)

        Calculate all weapon/ring combos and find cost/per damage taken

        X Find the greatest common multiple of 103 and 100 = 103*100

        For each armor/ring combo calculate how many turns (but -1 because u go first)
        it will take him to do 100 damage. You will need to do 104 damage in this
        many turns. 
            So for each of the weapon/ring combo calculate the turns/cost to do
            104+ damage in that many turns. Return the minimum one
            
*/

fn main() {
    let weapons : Vec<(i32,i32)> = vec![(8,4), (10,5), (25, 6), (40, 7), (74,8)];
    let armor : Vec<(i32,i32)> = vec![(0, 0), (13,1), (31,2), (53,3), (75,4), (102,5)];
    let offense_ring: Vec<(i32,i32)> = vec![(0,0), (25,1), (50,2), (100,3)];
    let defense_ring: Vec<(i32,i32)> = vec![(0,0), (20,1), (40,2), (80,3)];

    let mut minprice = f32::MAX;
    for a in armor.iter() {
        for d in defense_ring.iter() {
            let bossdmg : f32 = 9.0 - (a.1 as f32) - (d.1 as f32);
            let turns_till_player_dies = (100.0 / bossdmg).ceil();
            println!("armor: {} ring: {} bossdmg: {} turns: {}", a.1, d.1, bossdmg, turns_till_player_dies); 
            for w in weapons.iter() {
                for o in offense_ring.iter() {
                    let playerdmg : f32 = (w.1 as f32) + (o.1 as f32) - 2.0;
                    let turns_till_boss_dies = (104.0 / playerdmg).ceil();
                    if (turns_till_boss_dies as i32) <= (turns_till_player_dies as i32) {
                        let price : f32 = (a.0 as f32) + (d.0 as f32) + (w.0 as f32) + (o.0 as f32);
                        if price <= minprice {
                            minprice = price;
                        }
                    }
                }
            }
        }
    }

    println!("minimum cost to win: {}", minprice);

  
}
