use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

pub struct D22;

#[derive(Clone)]
struct Effect {
    name: String,
    armor: i32,
    damage: i32,
    mana: i32,
    duration: i32,
}

#[derive(Clone)]
struct Spell {
    name: String,
    cost: i32,
    damage: i32,
    heal: i32,
    effect: Option<Effect>,
}

impl Solve for D22 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut boss_stats: HashMap<String, i32> = HashMap::new();
        for stat in inputs {
            let stats: Vec<String> = stat.split(": ").map(|x| x.to_string()).collect();
            boss_stats.insert(stats[0].clone(), stats[1].parse::<i32>().unwrap());
        }

        let spells_and_effects: (HashMap<String, Spell>, HashMap<String, Effect>) = setup();

        let mut scores: Vec<i32> = vec![];
        let player = (50, 500);
        let boss = (
            boss_stats[&"Hit Points".to_string()],
            boss_stats[&"Damage".to_string()],
        );
        let active_effects: HashMap<String, i32> = HashMap::from([
            ("Shield".to_string(), 0),
            ("Poison".to_string(), 0),
            ("Recharge".to_string(), 0),
        ]);
        fight(
            spells_and_effects.0,
            spells_and_effects.1,
            player,
            boss,
            active_effects,
            1,
            0,
            &mut scores,
            false,
        );

        let result = scores.iter().min().unwrap();

        return result.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut boss_stats: HashMap<String, i32> = HashMap::new();
        for stat in inputs {
            let stats: Vec<String> = stat.split(": ").map(|x| x.to_string()).collect();
            boss_stats.insert(stats[0].clone(), stats[1].parse::<i32>().unwrap());
        }

        let spells_and_effects: (HashMap<String, Spell>, HashMap<String, Effect>) = setup();

        let mut scores: Vec<i32> = vec![];
        let player = (50, 500);
        let boss = (
            boss_stats[&"Hit Points".to_string()],
            boss_stats[&"Damage".to_string()],
        );
        let active_effects: HashMap<String, i32> = HashMap::from([
            ("Shield".to_string(), 0),
            ("Poison".to_string(), 0),
            ("Recharge".to_string(), 0),
        ]);
        fight(
            spells_and_effects.0,
            spells_and_effects.1,
            player,
            boss,
            active_effects,
            1,
            0,
            &mut scores,
            true,
        );

        let result = scores.iter().min().unwrap();

        return result.to_string();
    }
}

fn setup() -> (HashMap<String, Spell>, HashMap<String, Effect>) {
    let magic_missile = Spell {
        name: "Magic Missile".to_string(),
        cost: 53,
        damage: 4,
        heal: 0,
        effect: None,
    };

    let drain = Spell {
        name: "Drain".to_string(),
        cost: 73,
        damage: 2,
        heal: 2,
        effect: None,
    };

    let shiled_effect = Effect {
        name: "Shield".to_string(),
        armor: 7,
        mana: 0,
        damage: 0,
        duration: 6,
    };
    let shield = Spell {
        name: "Shield".to_string(),
        cost: 113,
        damage: 0,
        heal: 0,
        effect: Some(shiled_effect.clone()),
    };

    let poison_effect = Effect {
        name: "Poison".to_string(),
        armor: 0,
        mana: 0,
        damage: 3,
        duration: 6,
    };
    let poison = Spell {
        name: "Poison".to_string(),
        cost: 173,
        damage: 0,
        heal: 0,
        effect: Some(poison_effect.clone()),
    };

    let recharge_effect = Effect {
        name: "Recharge".to_string(),
        armor: 0,
        mana: 101,
        damage: 0,
        duration: 5,
    };
    let recharge = Spell {
        name: "Recharge".to_string(),
        cost: 229,
        damage: 0,
        heal: 0,
        effect: Some(recharge_effect.clone()),
    };
    let spells = HashMap::from([
        ("Magic Missile".to_string(), magic_missile),
        ("Drain".to_string(), drain),
        ("Shield".to_string(), shield),
        ("Poison".to_string(), poison),
        ("Recharge".to_string(), recharge),
    ]);

    let effects = HashMap::from([
        ("Shield".to_string(), shiled_effect),
        ("Poison".to_string(), poison_effect),
        ("Recharge".to_string(), recharge_effect),
    ]);
    return (spells, effects);
}

fn fight(
    spells: HashMap<String, Spell>,
    effects: HashMap<String, Effect>,
    mut player: (i32, i32),
    mut boss: (i32, i32),
    mut active_effects: HashMap<String, i32>,
    turn: i32,
    cost: i32,
    scores: &mut Vec<i32>,
    hard_mode: bool,
) {
    if hard_mode && turn % 2 != 0 {
        player.0 -= 1;
    }
    for key in effects.keys() {
        let effect = effects[key].clone();
        if *active_effects.get(key).unwrap() > 0 {
            boss.0 -= effect.damage;
            player.1 += effect.mana;
            active_effects.entry(key.clone()).and_modify(|x| *x -= 1);
        }
    }
    if player.0 <= 0 {
        return;
    }
    if boss.0 <= 0 {
        scores.push(cost);
        return;
    }
    if player.1 < 53 {
        return;
    }
    if turn % 2 == 0 {
        let mut boss_damage = boss.1;
        if *active_effects.get("Shield").unwrap() > 0 {
            boss_damage -= effects.get("Shield").unwrap().armor;
        }
        player.0 -= boss_damage;
        fight(
            spells,
            effects,
            player.clone(),
            boss.clone(),
            active_effects.clone(),
            turn + 1,
            cost,
            scores,
            hard_mode,
        );
    } else {
        for key in spells.keys() {
            let spell = spells[key].clone();
            let mut new_player = player.clone();
            let mut new_boss = boss.clone();
            let mut new_active_effects = active_effects.clone();
            if active_effects.contains_key(&spell.name)
                && *active_effects.get(&spell.name).unwrap() > 0
            {
                continue;
            }

            if new_player.1 < spell.cost {
                continue;
            }

            new_player.1 -= spell.cost;
            new_player.0 += spell.heal;
            new_boss.0 -= spell.damage;
            if spell.effect.is_some() {
                let effect = spell.effect.unwrap();
                new_active_effects
                    .entry(effect.name)
                    .and_modify(|x| *x = effect.duration);
            }

            fight(
                spells.clone(),
                effects.clone(),
                new_player,
                new_boss,
                new_active_effects,
                turn + 1,
                cost + spell.cost,
                scores,
                hard_mode,
            );
        }
    }
}
