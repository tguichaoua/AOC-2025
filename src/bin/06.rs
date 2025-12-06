use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

fn parse(input: &str) -> (Vec<Vec<u64>>, impl Iterator<Item = Op>) {
    let (numbers, ops) = input.rsplit_once('\n').unwrap();

    let number_of_cols = numbers
        .lines()
        .next()
        .map(|line| line.split_ascii_whitespace().count())
        .unwrap();

    let mut problems = vec![Vec::new(); number_of_cols];
    numbers.lines().for_each(|line| {
        line.split_ascii_whitespace()
            .enumerate()
            .for_each(|(col, n)| {
                let n: u64 = n.parse().unwrap();
                problems.get_mut(col).unwrap().push(n);
            })
    });

    let ops = ops
        .split_ascii_whitespace()
        .map(|ops| match ops.as_bytes()[0] {
            b'+' => Op::Add,
            b'*' => Op::Mul,
            _ => unreachable!(),
        });

    (problems, ops)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (problems, ops) = parse(input);

    let total = problems
        .iter()
        .zip(ops)
        .map(|(numbers, op)| match op {
            Op::Add => numbers.iter().sum::<u64>(),
            Op::Mul => numbers.iter().product::<u64>(),
        })
        .sum::<u64>();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (numbers, ops) = input.rsplit_once('\n').unwrap();
    let line_size = input.lines().next().map(|line| line.len() + 1).unwrap();
    let line_count = (numbers.len() / line_size) + 1;

    let read = |row: usize, col: usize| {
        let i = row * line_size + col;
        numbers.as_bytes()[i]
    };

    let ops = ops.as_bytes().iter().peekable().batching(|bytes| {
        let op = match bytes.next()? {
            b'+' => Op::Add,
            b'*' => Op::Mul,
            _ => unreachable!(),
        };

        let mut w = 0_usize;
        while bytes.next_if(|b| b.is_ascii_whitespace()).is_some() {
            w += 1
        }
        if bytes.peek().is_none() {
            w += 1;
        }

        Some((op, w))
    });

    let mut total = 0;
    let mut col = 0;
    for (op, w) in ops {
        let mut col_total = match op {
            Op::Add => 0,
            Op::Mul => 1,
        };

        for c in col..(col + w) {
            let mut s = Vec::new();
            for r in 0..line_count {
                s.push(read(r, c));
            }
            let s = unsafe { str::from_utf8_unchecked(&s) };
            let n: u64 = s.trim().parse().unwrap();

            match op {
                Op::Add => col_total += n,
                Op::Mul => col_total *= n,
            }
        }

        total += col_total;
        col += w + 1;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
