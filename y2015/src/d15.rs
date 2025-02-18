use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D15;

impl Solve for D15 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut ingredients: HashMap<String, HashMap<String, i32>> = HashMap::new();
        for input in inputs {
            let data: Vec<String> = input.split(": ").map(|x| x.to_string()).collect();
            let name = data[0].clone();
            let properties: Vec<String> = data[1].split(", ").map(|x| x.to_string()).collect();

            for prop in properties {
                let p: Vec<String> = prop.split(' ').map(|x| x.to_string()).collect();
                let prop_name = p[0].clone();
                let val: i32 = p[1].clone().parse().unwrap();

                ingredients
                    .entry(name.clone())
                    .and_modify(|i| {
                        i.insert(prop_name.clone(), val);
                    })
                    .or_insert(HashMap::from([(prop_name, val)]));
            }
        }

        let mut score = 0;
        for sp in 0..=100 {
            for pe in 0..=100 {
                for fr in 0..=100 {
                    for su in 0..=100 {
                        let s = calculate(&ingredients, sp, pe, fr, su);
                        if s > score {
                            score = s;
                        }
                    }
                }
            }
        }

        return score.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut ingredients: HashMap<String, HashMap<String, i32>> = HashMap::new();
        for input in inputs {
            let data: Vec<String> = input.split(": ").map(|x| x.to_string()).collect();
            let name = data[0].clone();
            let properties: Vec<String> = data[1].split(", ").map(|x| x.to_string()).collect();

            for prop in properties {
                let p: Vec<String> = prop.split(' ').map(|x| x.to_string()).collect();
                let prop_name = p[0].clone();
                let val: i32 = p[1].clone().parse().unwrap();

                ingredients
                    .entry(name.clone())
                    .and_modify(|i| {
                        i.insert(prop_name.clone(), val);
                    })
                    .or_insert(HashMap::from([(prop_name, val)]));
            }
        }

        let mut score = 0;
        for sp in 0..=100 {
            for pe in 0..=100 {
                for fr in 0..=100 {
                    for su in 0..=100 {
                        let s = calculate_with_calories(&ingredients, sp, pe, fr, su);
                        if s > score {
                            score = s;
                        }
                    }
                }
            }
        }

        return score.to_string();
    }
}

fn calculate(
    ingredients: &HashMap<String, HashMap<String, i32>>,
    sprinkles: i32,
    peanutbutter: i32,
    frosting: i32,
    sugar: i32,
) -> i32 {
    if sprinkles + peanutbutter + frosting + sugar != 100 {
        return 0;
    }

    let capacity = (ingredients["Sprinkles"]["capacity"] * sprinkles)
        + (ingredients["PeanutButter"]["capacity"] * peanutbutter)
        + (ingredients["Frosting"]["capacity"] * frosting)
        + (ingredients["Sugar"]["capacity"] * sugar);
    if capacity <= 0 {
        return 0;
    }

    let durability = (ingredients["Sprinkles"]["durability"] * sprinkles)
        + (ingredients["PeanutButter"]["durability"] * peanutbutter)
        + (ingredients["Frosting"]["durability"] * frosting)
        + (ingredients["Sugar"]["durability"] * sugar);
    if durability <= 0 {
        return 0;
    }

    let flavor = (ingredients["Sprinkles"]["flavor"] * sprinkles)
        + (ingredients["PeanutButter"]["flavor"] * peanutbutter)
        + (ingredients["Frosting"]["flavor"] * frosting)
        + (ingredients["Sugar"]["flavor"] * sugar);
    if flavor <= 0 {
        return 0;
    }

    let texture = (ingredients["Sprinkles"]["texture"] * sprinkles)
        + (ingredients["PeanutButter"]["texture"] * peanutbutter)
        + (ingredients["Frosting"]["texture"] * frosting)
        + (ingredients["Sugar"]["texture"] * sugar);
    if texture <= 0 {
        return 0;
    }

    return capacity * durability * flavor * texture;
}

fn calculate_with_calories(
    ingredients: &HashMap<String, HashMap<String, i32>>,
    sprinkles: i32,
    peanutbutter: i32,
    frosting: i32,
    sugar: i32,
) -> i32 {
    if sprinkles + peanutbutter + frosting + sugar != 100 {
        return 0;
    }

    let calories = (ingredients["Sprinkles"]["calories"] * sprinkles)
        + (ingredients["PeanutButter"]["calories"] * peanutbutter)
        + (ingredients["Frosting"]["calories"] * frosting)
        + (ingredients["Sugar"]["calories"] * sugar);
    if calories != 500 {
        return 0;
    }

    let capacity = (ingredients["Sprinkles"]["capacity"] * sprinkles)
        + (ingredients["PeanutButter"]["capacity"] * peanutbutter)
        + (ingredients["Frosting"]["capacity"] * frosting)
        + (ingredients["Sugar"]["capacity"] * sugar);
    if capacity <= 0 {
        return 0;
    }

    let durability = (ingredients["Sprinkles"]["durability"] * sprinkles)
        + (ingredients["PeanutButter"]["durability"] * peanutbutter)
        + (ingredients["Frosting"]["durability"] * frosting)
        + (ingredients["Sugar"]["durability"] * sugar);
    if durability <= 0 {
        return 0;
    }

    let flavor = (ingredients["Sprinkles"]["flavor"] * sprinkles)
        + (ingredients["PeanutButter"]["flavor"] * peanutbutter)
        + (ingredients["Frosting"]["flavor"] * frosting)
        + (ingredients["Sugar"]["flavor"] * sugar);
    if flavor <= 0 {
        return 0;
    }

    let texture = (ingredients["Sprinkles"]["texture"] * sprinkles)
        + (ingredients["PeanutButter"]["texture"] * peanutbutter)
        + (ingredients["Frosting"]["texture"] * frosting)
        + (ingredients["Sugar"]["texture"] * sugar);
    if texture <= 0 {
        return 0;
    }

    return capacity * durability * flavor * texture;
}
