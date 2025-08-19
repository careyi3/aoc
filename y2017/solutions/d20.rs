use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use utils::{file_reader, harness::Solve};

pub struct D20;

impl Solve for D20 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut particles: Vec<Particle> = inputs
            .iter()
            .enumerate()
            .map(|(id, input)| Particle::new(id as i64, input))
            .collect();

        for _ in 0..350 {
            particles.iter_mut().for_each(|f| f.next());
            particles.sort();
        }

        return particles.first().unwrap().id.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut particles: Vec<Particle> = inputs
            .iter()
            .enumerate()
            .map(|(id, input)| Particle::new(id as i64, input))
            .collect();

        for _ in 0..50 {
            particles.iter_mut().for_each(|f| f.next());
            particles = strictly_unique(&particles);
        }

        return particles.len().to_string();
    }
}

fn strictly_unique<T: Eq + Hash + Clone>(items: &Vec<T>) -> Vec<T> {
    let mut counts = HashMap::new();

    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }

    items
        .clone()
        .into_iter()
        .filter(|x| counts.get(&x).copied() == Some(1))
        .collect()
}

#[derive(Clone, Ord, Eq)]
struct Particle {
    id: i64,
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
    ax: i64,
    ay: i64,
    az: i64,
}

impl Particle {
    pub fn new(id: i64, input: &String) -> Self {
        let stuff: Vec<String> = input
            .split(", ")
            .map(|x| x[3..x.len() - 1].to_string())
            .collect();
        let positions: Vec<i64> = stuff[0]
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let velocities: Vec<i64> = stuff[1]
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let accelerations: Vec<i64> = stuff[2]
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        Particle {
            id,
            x: positions[0],
            y: positions[1],
            z: positions[2],
            vx: velocities[0],
            vy: velocities[1],
            vz: velocities[2],
            ax: accelerations[0],
            ay: accelerations[1],
            az: accelerations[2],
        }
    }

    pub fn next(&mut self) {
        self.vx += self.ax;
        self.vy += self.ay;
        self.vz += self.az;

        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    pub fn dist(&self) -> i64 {
        return self.x.abs() + self.y.abs() + self.z.abs();
    }
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.dist().cmp(&other.dist()));
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Hash for Particle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}
