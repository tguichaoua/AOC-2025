#![expect(dead_code)]
use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<u64>>,
    joltages: Vec<u64>,
}

fn parse(input: &str) -> impl Iterator<Item = Machine> {
    input.lines().map(|line| {
        let line = line.strip_prefix('[').unwrap();
        let (lights, line) = line.split_once(']').unwrap();
        let mut buttons = line.split_ascii_whitespace();
        let joltages = buttons.next_back().unwrap();

        let lights = lights
            .bytes()
            .map(|b| match b {
                b'.' => false,
                b'#' => true,
                _ => unreachable!(),
            })
            .collect_vec();

        let buttons = buttons
            .map(|btn| {
                btn.strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let joltages = joltages
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect_vec();

        Machine {
            lights,
            buttons,
            joltages,
        }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .map(|machine| {
                let n_light = machine.lights.len();

                let buttons = machine
                    .buttons
                    .into_iter()
                    .map(|btn| {
                        let mut v = vec![false; n_light];
                        for i in btn {
                            v[usize::try_from(i).unwrap()] = true;
                        }
                        v
                    })
                    .collect_vec();

                let mut count = u32::MAX;

                let max = (1_u32 << buttons.len()) - 1;

                debug_assert_eq!(max.count_ones(), u32::try_from(buttons.len()).unwrap());

                for n in 0..=max {
                    let mut result = vec![false; n_light];

                    for (i, btn) in buttons.iter().enumerate() {
                        if (n & (1 << i)) != 0 {
                            result.iter_mut().zip(btn).for_each(|(res, x)| *res ^= x);
                        }
                    }

                    if result == machine.lights {
                        let n_ones = n.count_ones();
                        if n_ones < count {
                            count = n_ones;
                        }
                    }
                }

                u64::from(count)
            })
            .sum::<u64>(),
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    None

    // Some(
    //     parse(input)
    //         .map(|machine| {
    //             todo!();
    //         })
    //         .sum::<u64>(),
    // )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
        // assert_eq!(result, Some(33));
    }
}
