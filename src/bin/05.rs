use std::ops::RangeInclusive;

advent_of_code::solution!(5);

fn parse(
    input: &str,
) -> (
    impl Iterator<Item = RangeInclusive<u64>>,
    impl Iterator<Item = u64>,
) {
    let (fresh_range, ids) = input.split_once("\n\n").unwrap();

    let fresh_range = fresh_range.lines().map(|range| {
        let (a, b) = range.split_once('-').unwrap();
        let a: u64 = a.parse().unwrap();
        let b: u64 = b.parse().unwrap();

        a..=b
    });

    let ids = ids.lines().map(|id| {
        let id: u64 = id.parse().unwrap();
        id
    });

    (fresh_range, ids)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (fresh_ranges, ids) = parse(input);

    let fresh_ranges = fresh_ranges.collect::<Vec<_>>();

    let mut total = 0;
    for id in ids {
        if fresh_ranges.iter().any(|range| range.contains(&id)) {
            total += 1;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (fresh_ranges, _) = parse(input);

    let mut fresh_ranges = fresh_ranges.collect::<Vec<_>>();

    fresh_ranges.sort_by_key(|range| *range.start());

    let mut fresh_ranges = fresh_ranges.iter();

    let first = fresh_ranges.next().unwrap();

    let mut total = first.end() - first.start() + 1;
    let mut current = *first.end();

    for range in fresh_ranges {
        if current < *range.start() {
            total += 1;
            current = *range.start();
        }

        if current <= *range.end() {
            total += range.end() - current;
            current = *range.end();
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
