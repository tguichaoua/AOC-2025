use std::ops::RangeInclusive;

advent_of_code::solution!(2);

fn parse(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> + '_ {
    input.split(',').map(|range| {
        let (lower, upper) = range.split_once('-').unwrap();
        let lower: u64 = lower.parse().unwrap();
        let upper: u64 = upper.parse().unwrap();

        lower..=upper
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);

    let mut total = 0;
    for range in input {
        for x in range {
            let len_x = x.ilog10() + 1;
            if len_x % 2 != 0 {
                continue;
            }
            let i = len_x / 2;

            let div = 10_u64.strict_pow(i) + 1;

            if x % div != 0 {
                continue;
            }

            total += x;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse(input);

    let mut total = 0;
    for range in input {
        'numbers: for x in range {
            let len_x = x.ilog10() + 1;

            for k in 2..=len_x {
                if len_x % k != 0 {
                    continue;
                }
                let i = len_x / k;

                let mut div = 0;
                for j in 0..k {
                    div += 10_u64.strict_pow(j * i);
                }

                if x % div == 0 {
                    total += x;
                    continue 'numbers;
                }
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
