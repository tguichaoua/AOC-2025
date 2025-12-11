use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(11);

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|line| {
            let (name, outputs) = line.split_once(':').unwrap();
            let outputs = outputs
                .split_ascii_whitespace()
                .map(|s| s.to_owned())
                .collect_vec();
            let name = name.to_owned();

            (name, outputs)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse(input);

    let mut flows = VecDeque::new();
    flows.push_back("you");

    let mut path_to_out = 0;

    while let Some(node) = flows.pop_front() {
        let outputs = graph.get(node).unwrap();

        for output in outputs {
            if output == "out" {
                path_to_out += 1;
            } else {
                flows.push_back(output.as_str());
            }
        }
    }

    Some(path_to_out)
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse(input);

    let n_f_d = num_path_to(&graph, "fft", "dac");
    let n_d_f = num_path_to(&graph, "dac", "fft");

    let mut n_paths = 0;

    if n_f_d != 0 {
        let n_s_f = num_path_to(&graph, "svr", "fft");
        let n_d_o = num_path_to(&graph, "dac", "out");

        n_paths += n_s_f * n_f_d * n_d_o;
    }

    if n_d_f != 0 {
        let n_s_d = num_path_to(&graph, "svr", "dac");
        let n_f_o = num_path_to(&graph, "fft", "out");

        n_paths += n_s_d * n_d_f * n_f_o;
    }

    Some(n_paths)
}

fn num_path_to(graph: &HashMap<String, Vec<String>>, start: &str, end: &str) -> u64 {
    let mut visitor_in = HashMap::new();

    {
        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            let Some(outputs) = graph.get(node) else {
                continue;
            };

            for output in outputs {
                match visitor_in.entry(output.as_str()) {
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(HashSet::new()).insert(node);
                        if output != end {
                            queue.push_back(output.as_str());
                        }
                    }
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        entry.get_mut().insert(node);
                    }
                }
            }
        }
    }

    let mut n_path = 0;

    {
        let mut queue = VecDeque::new();
        queue.push_back(end);

        while let Some(node) = queue.pop_front() {
            let Some(visitors) = visitor_in.get(node) else {
                continue;
            };

            for v in visitors {
                if *v == start {
                    n_path += 1;
                } else {
                    queue.push_back(v);
                }
            }
        }
    }
    // let mut flows = VecDeque::new();
    // flows.push_back(start);

    // let mut n_path_to_end = 0;

    // let mut visited = HashSet::new();

    // while let Some(node) = flows.pop_front() {
    //     let Some(outputs) = graph.get(node) else {
    //         continue;
    //     };

    //     for output in outputs {
    //         if output == end {
    //             n_path_to_end += 1;
    //         } else {
    //             if !visited.insert((node, output)) {
    //                 continue;
    //             }

    //             flows.push_back(output.as_str());
    //         }
    //     }
    // }

    n_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
