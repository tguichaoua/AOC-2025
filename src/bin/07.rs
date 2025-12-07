use std::{collections::HashSet, mem};

use advent_of_code::utils::ascii_map::{ascii_map_size, parse_ascii_map};
use glam::{UVec2, uvec2};

advent_of_code::solution!(7);

struct Input {
    start: UVec2,
    size: UVec2,
    splitters: HashSet<UVec2>,
}

fn parse(input: &str) -> Input {
    let mut start = None;
    let mut splitters = HashSet::new();
    let size = ascii_map_size(input);
    parse_ascii_map(input).for_each(|(pos, b)| {
        match b {
            b'S' => {
                start = Some(pos);
            }
            b'^' => {
                splitters.insert(pos);
            }
            _ => { /* empty */ }
        }
    });

    Input {
        start: start.unwrap(),
        size,
        splitters,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let Input {
        start,
        size,
        splitters,
    } = parse(input);

    let mut y = start.y;
    let mut beams = vec![start.x];
    let mut next_beams = Vec::new();

    let mut num_of_split = 0;

    fn push_beam(beams: &mut Vec<u32>, new_beam: u32) {
        if !beams.contains(&new_beam) {
            beams.push(new_beam);
        }
    }

    loop {
        if y >= size.y {
            break;
        }

        y += 1;

        for beam_x in beams.drain(..) {
            if splitters.contains(&uvec2(beam_x, y)) {
                num_of_split += 1;

                if let Some(x) = beam_x.checked_sub(1) {
                    push_beam(&mut next_beams, x);
                }

                let x = beam_x + 1;
                if x < size.x {
                    push_beam(&mut next_beams, x);
                }
            } else {
                push_beam(&mut next_beams, beam_x);
            }
        }

        mem::swap(&mut beams, &mut next_beams);
    }

    Some(num_of_split)
}

pub fn part_two(input: &str) -> Option<u64> {
    let Input {
        start,
        size,
        splitters,
    } = parse(input);

    struct Beam {
        x: u32,
        timelines: u64,
    }

    let mut y = start.y;
    let mut beams = vec![Beam {
        x: start.x,
        timelines: 1,
    }];
    let mut next_beams = Vec::new();

    fn push_beam(beams: &mut Vec<Beam>, new_beam: Beam) {
        if let Some(beam) = beams.iter_mut().find(|beam| beam.x == new_beam.x) {
            beam.timelines += new_beam.timelines;
        } else {
            beams.push(new_beam);
        }
    }

    loop {
        if y >= size.y {
            break;
        }

        y += 1;

        for beam in beams.drain(..) {
            if splitters.contains(&uvec2(beam.x, y)) {
                if let Some(x) = beam.x.checked_sub(1) {
                    push_beam(
                        &mut next_beams,
                        Beam {
                            x,
                            timelines: beam.timelines,
                        },
                    );
                }

                let x = beam.x + 1;
                if x < size.x {
                    push_beam(
                        &mut next_beams,
                        Beam {
                            x,
                            timelines: beam.timelines,
                        },
                    );
                }
            } else {
                push_beam(&mut next_beams, beam);
            }
        }

        mem::swap(&mut beams, &mut next_beams);
    }

    Some(beams.iter().map(|beam| beam.timelines).sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
