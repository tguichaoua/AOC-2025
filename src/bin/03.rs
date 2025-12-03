use advent_of_code::utils::iter::IterExt;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let bat = line.as_bytes();

                let (i, d) = bat[..bat.len() - 1]
                    .iter()
                    .enumerate()
                    .first_max_by_key(|(_, x)| *x)
                    .unwrap();

                let u = bat[i + 1..].iter().first_max_by_key(|x| *x).unwrap();

                (d - b'0') * 10 + (u - b'0')
            })
            .map(u64::from)
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut bat = line.as_bytes();
                let mut jolt = 0;

                for p in 0..12_u32 {
                    let max = bat.len().checked_sub(11 - usize::try_from(p).unwrap());

                    match max {
                        None | Some(0) => {
                            jolt += bat
                                .iter()
                                .enumerate()
                                .map(|(i, d)| {
                                    u64::from(d - b'0')
                                        * 10_u64.pow(11 - p - u32::try_from(i).unwrap())
                                })
                                .sum::<u64>();
                        }
                        Some(max) => {
                            let (i, d) = bat[..max]
                                .iter()
                                .enumerate()
                                .first_max_by_key(|(_, x)| *x)
                                .unwrap();
                            bat = &bat[i + 1..];

                            jolt += u64::from(d - b'0') * 10_u64.pow(11 - p);
                        }
                    }
                }

                jolt
            })
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
