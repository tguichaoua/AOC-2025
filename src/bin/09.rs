use std::ops::RangeInclusive;

use glam::*;
use itertools::Itertools;

advent_of_code::solution!(9);

fn parse(input: &str) -> impl Iterator<Item = U64Vec2> {
    input.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        u64vec2(x, y)
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let tiles = parse(input).collect_vec();

    Some(
        tiles
            .iter()
            .enumerate()
            .flat_map(|(i, a)| tiles[i + 1..].iter().map(move |b| (a, b)))
            .map(|(a, b)| {
                let w = a.x.abs_diff(b.x) + 1;
                let h = a.y.abs_diff(b.y) + 1;
                w * h
            })
            .max()
            .unwrap(),
    )
}

enum Segment {
    H { x: RangeInclusive<u64>, y: u64 },
    V { x: u64, y: RangeInclusive<u64> },
}

impl Segment {
    fn contains(&self, p: U64Vec2) -> bool {
        match self {
            Segment::H { x, y } => &p.y == y && x.contains(&p.x),
            Segment::V { x, y } => &p.x == x && y.contains(&p.y),
        }
    }
}

pub fn part_two(_input: &str) -> Option<u64> {
    None

    // let tiles = parse(input).collect_vec();

    // let segments = tiles
    //     .iter()
    //     .circular_tuple_windows()
    //     .map(|(&a, &b)| points_to_segment(a, b))
    //     .collect_vec();

    // let rectangles = tiles
    //     .iter()
    //     .enumerate()
    //     .flat_map(|(i, a)| tiles[i + 1..].iter().map(move |b: &U64Vec2| (a, b)));

    // let biggest_rect_area = rectangles
    //     .filter(|(a, b)| rect_edge(**a, **b).all(|p| is_inside(&segments, p)))
    //     .map(|(a, b)| {
    //         let w = a.x.abs_diff(b.x) + 1;
    //         let h = a.y.abs_diff(b.y) + 1;
    //         w * h
    //     })
    //     .max()
    //     .unwrap();

    // Some(biggest_rect_area)
}

fn points_to_segment(a: U64Vec2, b: U64Vec2) -> Segment {
    if a.x == b.x {
        let y = if a.y < b.y { a.y..=b.y } else { b.y..=a.y };
        Segment::V { x: a.x, y }
    } else if a.y == b.y {
        let x = if a.x < b.x { a.x..=b.x } else { b.x..=a.x };
        Segment::H { x, y: a.y }
    } else {
        unreachable!();
    }
}

// TODO: replace this by returning 4 segments
fn rect_edge(a: U64Vec2, b: U64Vec2) -> impl Iterator<Item = U64Vec2> {
    let min = a.min(b);
    let max = a.max(b);

    let top = (min.x..=max.x).map(move |x| u64vec2(x, min.y));
    let bot = (min.x..=max.x).map(move |x| u64vec2(x, max.y));
    let left = (min.y..=max.y).map(move |y| u64vec2(min.x, y));
    let right = (min.y..=max.y).map(move |y| u64vec2(max.x, y));

    top.chain(right).chain(bot).chain(left)
}

fn is_inside(segments: &[Segment], point: U64Vec2) -> bool {
    if segments.iter().any(|seg| seg.contains(point)) {
        return true;
    }

    let segments_above = segments.iter().filter(|seg| match seg {
        Segment::H { x, y } => y <= &point.y && x.contains(&point.x),
        Segment::V { x, y } => x == &point.x && y.start() <= &point.y,
    });

    segments_above.count() % 2 == 1
}

// fn is_seg_inside(perimeter: &[Segment], seg: Segment) -> bool {
//     match seg {
//         Segment::H { x, y } => {
//             for s in perimeter {
//                 match s {
//                     Segment::H { x, y } => todo!(),
//                     Segment::V { x: px, y: py } => px > x.start() && px < x.end() && todo!(),
//                 }
//             }
//         }
//         Segment::V { x, y } => todo!(),
//     }
//
//     todo!();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
