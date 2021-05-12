use std::collections::HashMap;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
#[derive(Debug, Clone)]
struct Spell {
    name: String,
    cost: i32,
    damage: i32,
    hp_heal: i32,
    mana_regen: i32,
    armor_buff: i32,
    duration: i32,
}
/*
 unsafe {
    srand();
    println!("{}", rand());
  }
}

extern "C" {
  fn srand() -> u32;
  fn rand() -> u32;
}
*/
impl Spell {
    fn new(proto: &Spell) -> Spell {
        Spell {
            name: proto.name.clone(),
            cost: proto.cost,
            damage: proto.damage,
            hp_heal: proto.hp_heal,
            mana_regen: proto.mana_regen,
            armor_buff: proto.armor_buff,
            duration: proto.duration
        }
    }
}

struct Player {
    mana: i32,
    hp: i32,
    armor: i32,
    activespells: HashMap<usize, Spell>,
    turn: bool,
    mana_used: i32
}

impl Player {
    fn new() -> Player {
        Player {
        mana: 500,
        hp: 50,
        armor: 0,
        activespells: HashMap::new(),
        turn: true,
        mana_used: 0
        }
    }
}




fn main() {
    let mut rng = rand::thread_rng();
  
    let mut spell_prototypes: Vec<Spell> = vec![];

    // Do nothing spell
    spell_prototypes.push(Spell{ name: "Do nothing".to_string(),cost: 0, damage: 0, hp_heal: 0, mana_regen: 0, armor_buff: 0, duration: 0});
    spell_prototypes.push(Spell{ name: "Magic missile".to_string(), cost: 53, damage: 4, hp_heal: 0, mana_regen: 0, armor_buff: 0, duration: 0});
    spell_prototypes.push(Spell{ name: "Magic missile".to_string(), cost: 53, damage: 4, hp_heal: 0, mana_regen: 0, armor_buff: 0, duration: 0});
    spell_prototypes.push(Spell{ name: "Drain".to_string(), cost: 73, damage: 2, hp_heal: 2, mana_regen: 0, armor_buff: 0, duration: 0});
    spell_prototypes.push(Spell{ name: "Shield".to_string(), cost: 113, damage: 0, hp_heal: 0, mana_regen: 0, armor_buff: 7, duration: 5});
    spell_prototypes.push(Spell{ name: "Shield".to_string(), cost: 113, damage: 0, hp_heal: 0, mana_regen: 0, armor_buff: 7, duration: 5});
    spell_prototypes.push(Spell{ name: "Poison".to_string(), cost: 173, damage: 3, hp_heal: 0, mana_regen: 0, armor_buff: 0, duration: 5});
    spell_prototypes.push(Spell{ name: "Poison".to_string(), cost: 173, damage: 3, hp_heal: 0, mana_regen: 0, armor_buff: 0, duration: 5});
    spell_prototypes.push(Spell{ name: "Recharge".to_string(), cost: 229, damage: 0, hp_heal: 0, mana_regen: 101, armor_buff: 0, duration: 4});
    spell_prototypes.push(Spell{ name: "Recharge".to_string(), cost: 229, damage: 0, hp_heal: 0, mana_regen: 101, armor_buff: 0, duration: 4});
    spell_prototypes.push(Spell{ name: "Recharge".to_string(), cost: 229, damage: 0, hp_heal: 0, mana_regen: 101, armor_buff: 0, duration: 4});

   
    // Seed random number generate
    let die = Uniform::from(0..spell_prototypes.len());
    const MAX_SIMS: u32 = 100000000;
    const BOSS_DAMAGE: i32 = 9;
    const BOSS_STARTING_HP: i32 = 51;

   
    let mut mincost: i32 = i32::MAX;
    for _ in 0..MAX_SIMS {
        let mut player = Player::new();
        let mut bosshp = BOSS_STARTING_HP;
        
        while player.hp >= 1 && bosshp >= 1 && player.mana >= 1 {
            // Player turn
            if player.turn {
                // println!("-- Player turn -- ");
            } else {
                // println!("-- Boss turn -- ");
            }

            // Do active spells, if it is finished, remove it from active spells set
            // Keep track of spells to remove
            let mut finishedspells: Vec<usize> = vec![];

            // Loop through all active spells, applying the spell
            let activespellkeys: Vec<usize> = player.activespells.keys().cloned().collect();
            for k in activespellkeys.iter() {
                let cur_spell = player.activespells.get(k).unwrap();
                if cur_spell.mana_regen != 0 {
                    player.mana += cur_spell.mana_regen;
                }
                if cur_spell.damage != 0 {
                    bosshp -= cur_spell.damage;
                }

                player.activespells.entry(*k).and_modify(|e| {
                    // println!("{}'s timer is now {}", e.name, e.duration);
                    (*e).duration -= 1;
                    if (*e).duration == 0 {
                        //  println!("{}'s is over.", e.name);
                        finishedspells.push(*k);
                    }
                });
            }



            // Remove any spells that completed
            for f in finishedspells.iter() {
                if player.activespells.get(f).unwrap().armor_buff != 0 {
                    player.armor = 0;
                }
                player.activespells.remove(f);
            }

            if player.turn {
                // Player's turn

                // Pick random spell to cast, if it's not instant, add it to active spells
                //let mut spellidx: usize = rand::thread_rng().gen_range(0..spell_prototypes.len()) as usize; // TODO: use rand
                let mut spellidx: usize = die.sample(&mut rng) as usize; 
                die.sample(&mut rng);
                let mut chosen_spell = &spell_prototypes[spellidx];

                loop {
                    while player.activespells.contains_key(&spellidx) ||
                        (player.mana - chosen_spell.cost) < 0 {
                            //spellidx = rand::thread_rng().gen_range(0..spell_prototypes.len()) as usize;
                            spellidx = die.sample(&mut rng) as usize; 
                            chosen_spell = &spell_prototypes[spellidx];
                        }
                    break;
                }

                if chosen_spell.duration == 0 {
                    // If spell is instant cast, do it right away
                    if chosen_spell.damage != 0 {
                        // println!("Casting {}, does {} damage.", chosen_spell.name, chosen_spell.damage);
                        bosshp -= chosen_spell.damage;
                    }
                    if chosen_spell.hp_heal != 0 {
                        // println!("Casting {}, heals {} hp.", chosen_spell.name, chosen_spell.hp_heal);
                        player.hp += chosen_spell.hp_heal;
                    }

                    if chosen_spell.name == "Do nothing" {
                        // println!("Not casting anything");
                    }

                } else {
                    let castspell = Spell::new(&chosen_spell);
                    // println!("Casting {}", castspell.name);
                    if castspell.armor_buff != 0 {
                        player.armor = castspell.armor_buff;
                    }
                    player.activespells.insert(spellidx, castspell);

                    
                }

                player.mana_used += chosen_spell.cost;
                player.mana -= chosen_spell.cost;
                
            } else {
                // Boss's turn
                // println!("Boss does {} - {}. Player hp : {}", 
                //     BOSS_DAMAGE, 
                //     (player.armor),
                //     player.hp - (BOSS_DAMAGE - (player.armor )));

                player.hp = player.hp - (BOSS_DAMAGE - (player.armor));
            }
            // println!("player hp: {} mp: {}   boss_hp: {}\n\n", player.hp, player.mana, bosshp);
             player.turn = !player.turn;
        }
// 
        // println!("Game ended. Player hp: {}, mp: {}, Boss hp: {}", player.hp, player.mana, bosshp);
        // println!("Mana used: {}", player.mana_used);
        if player.hp >= 1 &&  bosshp <= 0 && player.mana_used < mincost {
            println!("Player wins!");
            mincost = player.mana_used;
        }
    }

    println!("Minimum mana: {}", mincost);
}
