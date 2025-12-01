advent_of_code::solution!(1);

fn parse(input: &str) -> impl Iterator<Item = i64> + '_ {
    input.lines().map(|line| {
        let (sign, amount) = line.split_at(1);
        let amount: i64 = amount.parse().unwrap();
        match sign {
            "L" => -amount,
            _ => amount,
        }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut current = 50_i64;
    let mut zeroes = 0;

    for inst in parse(input) {
        current += inst;
        if (current % 100) == 0 {
            zeroes += 1;
        }
    }

    Some(zeroes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut current = 50_i64;
    let mut zeroes = 0;

    for inst in parse(input) {
        if (current % 100) == 0 {
            zeroes += (inst / 100).unsigned_abs();
        } else if inst < 0 {
            let mut delta = current % 100;
            if current < 0 {
                delta += 100;
            }

            if inst.abs() >= delta {
                zeroes += 1 + ((inst + delta) / 100).unsigned_abs();
            }
        } else {
            let mut delta = current % 100;
            if current < 0 {
                delta += 100;
            }

            let delta = 100 - delta;

            if inst >= delta {
                zeroes += 1 + ((inst - delta) / 100).unsigned_abs();
            }
        }

        current += inst;
    }

    Some(zeroes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
