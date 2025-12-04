use std::collections::HashSet;

use advent_of_code::utils::{
    ascii_map::{ascii_map_size, parse_ascii_map},
    vec::height_directions_bounded,
};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let map_size = ascii_map_size(input);

    let roll_positions = parse_ascii_map(input)
        .filter(|(_, b)| *b == b'@')
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>();

    let mut total = 0;
    for pos in &roll_positions {
        let n = height_directions_bounded(*pos, map_size)
            .filter(|p| roll_positions.contains(p))
            .count();

        if n < 4 {
            total += 1;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map_size = ascii_map_size(input);

    let mut roll_positions = parse_ascii_map(input)
        .filter(|(_, b)| *b == b'@')
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>();

    let count_start = roll_positions.len();

    let mut to_remove = Vec::new();

    loop {
        to_remove.clear();

        for pos in &roll_positions {
            let n = height_directions_bounded(*pos, map_size)
                .filter(|p| roll_positions.contains(p))
                .count();

            if n < 4 {
                to_remove.push(*pos);
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for p in &to_remove {
            roll_positions.remove(p);
        }
    }

    Some(u64::try_from(count_start - roll_positions.len()).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
