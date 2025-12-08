use std::collections::HashSet;

use glam::{I64Vec3, i64vec3};
use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Couple {
    a: I64Vec3,
    b: I64Vec3,
    dist: i64,
}

fn parse(input: &str) -> Vec<I64Vec3> {
    input
        .lines()
        .map(|line| {
            let (x, line) = line.split_once(',').unwrap();
            let (y, z) = line.split_once(',').unwrap();

            let x: i64 = x.parse().unwrap();
            let y: i64 = y.parse().unwrap();
            let z: i64 = z.parse().unwrap();

            i64vec3(x, y, z)
        })
        .collect()
}

fn compute_couple(boxes: &[I64Vec3]) -> Vec<Couple> {
    let mut couples = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, a)| boxes[i + 1..].iter().map(move |b| (a, b)))
        .map(|(&a, &b)| {
            let dist = a.distance_squared(b);
            Couple { a, b, dist }
        })
        .collect_vec();

    couples.sort_unstable_by_key(|couple| couple.dist);

    couples
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve_one(input, 1000))
}

fn solve_one(input: &str, count: usize) -> u64 {
    let boxes = parse(input);
    let couples = compute_couple(&boxes);

    let mut circuits = Vec::<HashSet<I64Vec3>>::new();

    for couple in couples.iter().take(count) {
        let circuit_a_pos = circuits
            .iter()
            .position(|circuit| circuit.contains(&couple.a));
        let circuit_b_pos = circuits
            .iter()
            .position(|circuit| circuit.contains(&couple.b));

        match (circuit_a_pos, circuit_b_pos) {
            (None, None) => {
                let mut circuit = HashSet::new();
                circuit.insert(couple.a);
                circuit.insert(couple.b);

                circuits.push(circuit);
            }
            (None, Some(i)) => {
                circuits.get_mut(i).unwrap().insert(couple.a);
            }
            (Some(i), None) => {
                circuits.get_mut(i).unwrap().insert(couple.b);
            }
            (Some(a), Some(b)) if a != b => {
                let [a, b] = circuits.get_disjoint_mut([a, b]).unwrap();
                a.extend(b.drain());
            }
            _ => { /* empty */ }
        }
    }

    circuits.sort_unstable_by_key(|circuit| circuit.len());

    circuits
        .iter()
        .rev()
        .map(|circuit| u64::try_from(circuit.len()).unwrap())
        .take(3)
        .product::<u64>()
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes = parse(input);
    let boxed_count = boxes.len();
    let couples = compute_couple(&boxes);

    let mut circuits = Vec::<HashSet<I64Vec3>>::new();

    for couple in &couples {
        let circuit_a_pos = circuits
            .iter()
            .position(|circuit| circuit.contains(&couple.a));
        let circuit_b_pos = circuits
            .iter()
            .position(|circuit| circuit.contains(&couple.b));

        let circuit = match (circuit_a_pos, circuit_b_pos) {
            (None, None) => {
                let mut circuit = HashSet::new();
                circuit.insert(couple.a);
                circuit.insert(couple.b);

                circuits.push(circuit);

                None
            }
            (None, Some(i)) => {
                let circuit = circuits.get_mut(i).unwrap();
                circuit.insert(couple.a);

                Some(circuit)
            }
            (Some(i), None) => {
                let circuit = circuits.get_mut(i).unwrap();
                circuit.insert(couple.b);

                Some(circuit)
            }
            (Some(a), Some(b)) if a != b => {
                let [a, b] = circuits.get_disjoint_mut([a, b]).unwrap();
                a.extend(b.drain());

                Some(a)
            }
            _ => None,
        };

        if let Some(circuit) = circuit
            && circuit.len() == boxed_count
        {
            return Some(u64::try_from(couple.a.x * couple.b.x).unwrap());
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_one(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
