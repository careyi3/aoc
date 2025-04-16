use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

use itertools::Itertools;

pub struct D21;

impl Solve for D21 {
    fn part1(_input: String, path: &String) -> String {
        let weapons: HashMap<String, (i32, i32, i32)> = HashMap::from([
            ("Dagger".to_string(), (8, 4, 0)),
            ("Shortsword".to_string(), (10, 5, 0)),
            ("Warhammer".to_string(), (25, 6, 0)),
            ("Longsword".to_string(), (40, 7, 0)),
            ("Greataxe".to_string(), (74, 8, 0)),
        ]);
        let armor: HashMap<String, (i32, i32, i32)> = HashMap::from([
            ("Leather".to_string(), (13, 0, 1)),
            ("Chainmail".to_string(), (31, 0, 2)),
            ("Splintmail".to_string(), (53, 0, 3)),
            ("Bandedmail".to_string(), (75, 0, 4)),
            ("Platemail".to_string(), (102, 0, 5)),
            ("Empty".to_string(), (0, 0, 0)),
        ]);
        let rings: HashMap<String, (i32, i32, i32)> = HashMap::from([
            ("Damage +1".to_string(), (25, 1, 0)),
            ("Damage +2".to_string(), (50, 2, 0)),
            ("Damage +3".to_string(), (100, 3, 0)),
            ("Defense +1".to_string(), (20, 0, 1)),
            ("Defense +2".to_string(), (40, 0, 2)),
            ("Defense +3".to_string(), (80, 0, 3)),
            ("Empty1".to_string(), (0, 0, 0)),
            ("Empty1".to_string(), (0, 0, 0)),
        ]);

        let inputs = file_reader::read_lines(path);
        let mut boss_stats: HashMap<String, i32> = HashMap::new();
        for stat in inputs {
            let stats: Vec<String> = stat.split(": ").map(|x| x.to_string()).collect();
            boss_stats.insert(stats[0].clone(), stats[1].parse::<i32>().unwrap());
        }

        let mut loadouts: Vec<(i32, i32, i32)> = vec![];
        for weapon in weapons.keys() {
            for piece in armor.keys() {
                for ring_pair in rings.keys().combinations(2) {
                    let mut loadout = (0, 0, 0);
                    loadout.0 += weapons[weapon].0;
                    loadout.1 += weapons[weapon].1;
                    loadout.2 += weapons[weapon].2;

                    loadout.0 += armor[piece].0;
                    loadout.1 += armor[piece].1;
                    loadout.2 += armor[piece].2;

                    for ring in ring_pair {
                        loadout.0 += rings[ring].0;
                        loadout.1 += rings[ring].1;
                        loadout.2 += rings[ring].2;
                    }
                    loadouts.push(loadout);
                }
            }
        }

        let boss = (
            boss_stats[&"Hit Points".to_string()],
            boss_stats[&"Damage".to_string()],
            boss_stats[&"Armor".to_string()],
        );

        let mut cost = 1000;
        for loadout in loadouts {
            let player = (100, loadout.1, loadout.2);
            if fight(player, boss) {
                if loadout.0 < cost {
                    cost = loadout.0;
                }
            }
        }

        return cost.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let weapons: HashMap<String, (i32, i32, i32)> = HashMap::from([
            ("Dagger".to_string(), (8, 4, 0)),
            ("Shortsword".to_string(), (10, 5, 0)),
            ("Warhammer".to_string(), (25, 6, 0)),
            ("Longsword".to_string(), (40, 7, 0)),
            ("Greataxe".to_string(), (74, 8, 0)),
        ]);
        let armor: HashMap<String, (i32, i32, i32)> = HashMap::from([
            ("Leather".to_string(), (13, 0, 1)),
            ("Chainmail".to_string(), (31, 0, 2)),
            ("Splintmail".to_string(), (53, 0, 3)),
            ("Bandedmail".to_string(), (75, 0, 4)),
            ("Platemail".to_string(), (102, 0, 5)),
            ("Empty".to_string(), (0, 0, 0)),
        ]);
        let rings: HashMap<String, (i32, i32, i32)> = HashMap::from([
            ("Damage +1".to_string(), (25, 1, 0)),
            ("Damage +2".to_string(), (50, 2, 0)),
            ("Damage +3".to_string(), (100, 3, 0)),
            ("Defense +1".to_string(), (20, 0, 1)),
            ("Defense +2".to_string(), (40, 0, 2)),
            ("Defense +3".to_string(), (80, 0, 3)),
            ("Empty1".to_string(), (0, 0, 0)),
            ("Empty1".to_string(), (0, 0, 0)),
        ]);

        let inputs = file_reader::read_lines(path);
        let mut boss_stats: HashMap<String, i32> = HashMap::new();
        for stat in inputs {
            let stats: Vec<String> = stat.split(": ").map(|x| x.to_string()).collect();
            boss_stats.insert(stats[0].clone(), stats[1].parse::<i32>().unwrap());
        }

        let mut loadouts: Vec<(i32, i32, i32)> = vec![];
        for weapon in weapons.keys() {
            for piece in armor.keys() {
                for ring_pair in rings.keys().combinations(2) {
                    let mut loadout = (0, 0, 0);
                    loadout.0 += weapons[weapon].0;
                    loadout.1 += weapons[weapon].1;
                    loadout.2 += weapons[weapon].2;

                    loadout.0 += armor[piece].0;
                    loadout.1 += armor[piece].1;
                    loadout.2 += armor[piece].2;

                    for ring in ring_pair {
                        loadout.0 += rings[ring].0;
                        loadout.1 += rings[ring].1;
                        loadout.2 += rings[ring].2;
                    }
                    loadouts.push(loadout);
                }
            }
        }

        let boss = (
            boss_stats[&"Hit Points".to_string()],
            boss_stats[&"Damage".to_string()],
            boss_stats[&"Armor".to_string()],
        );

        let mut cost = 0;
        for loadout in loadouts {
            let player = (100, loadout.1, loadout.2);
            if !fight(player, boss) {
                if loadout.0 > cost {
                    cost = loadout.0;
                }
            }
        }

        return cost.to_string();
    }
}

fn fight(player: (i32, i32, i32), boss: (i32, i32, i32)) -> bool {
    let mut boss_hp = boss.0;
    let mut player_hp = player.0;
    let mut player_turn = true;
    while boss_hp > 0 && player_hp > 0 {
        if player_turn {
            let mut damage = 1;
            if player.1 - boss.2 > 0 {
                damage = player.1 - boss.2;
            }
            boss_hp -= damage;
        } else {
            let mut damage = 1;
            if boss.1 - player.2 > 0 {
                damage = boss.1 - player.2;
            }
            player_hp -= damage;
        }

        player_turn = !player_turn;
    }
    if boss_hp <= 0 {
        return true;
    } else {
        return false;
    }
}
