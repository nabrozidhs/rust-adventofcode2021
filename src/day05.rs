use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;

fn _parse(input: &Vec<&str>, with_diagonals: bool) -> Vec<HashSet<(u64, u64)>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    }
    let mut output = vec![];
    for line in input.iter() {
        let cap = RE.captures(line).unwrap();
        let x1: u64 = cap[1].parse().unwrap();
        let y1: u64 = cap[2].parse().unwrap();
        let x2: u64 = cap[3].parse().unwrap();
        let y2: u64 = cap[4].parse().unwrap();
        let mut points: HashSet<(u64, u64)> = HashSet::new();
        if x1 == x2 {
            for y in y1.min(y2)..=y2.max(y1) {
                points.insert((x1, y));
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x2.max(x1) {
                points.insert((x, y1));
            }
        } else if with_diagonals {
            for index in 0..=(x1.max(x2) - x1.min(x2)) {
                points.insert(
                    (
                        (x1 as i64 + index as i64 * if x1 < x2 { 1 } else { -1 }) as u64,
                        (y1 as i64 + index as i64 * if y1 < y2 { 1 } else { -1 }) as u64,
                    ),
                );
            }
        }

        if !points.is_empty() {
            output.push(points);
        }
    }
    output
}

fn _day05(input: &Vec<&str>, with_diagonals: bool) -> usize {
    let lines = _parse(input, with_diagonals);
    let mut duplicates: HashSet<(u64, u64)> = HashSet::new();
    let mut discovered: HashSet<(u64, u64)> = HashSet::new();

    for line in lines {
        for point in line {
            if discovered.contains(&point) {
                duplicates.insert(point);
            } else {
                discovered.insert(point);
            }
        }
    }

    duplicates.len()
}

#[cfg(test)]
mod tests_day05 {
    use std::fs::File;
    use std::io::Read;

    use crate::day05::_day05;

    #[test]
    fn part1_test_input() {
        assert_eq!(5, _day05(&vec!["0,9 -> 5,9",
                                   "8,0 -> 0,8",
                                   "9,4 -> 3,4",
                                   "2,2 -> 2,1",
                                   "7,0 -> 7,4",
                                   "6,4 -> 2,0",
                                   "0,9 -> 2,9",
                                   "3,4 -> 1,4",
                                   "0,0 -> 8,8",
                                   "5,5 -> 8,2", ],
                             false));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(12, _day05(&vec!["0,9 -> 5,9",
                                    "8,0 -> 0,8",
                                    "9,4 -> 3,4",
                                    "2,2 -> 2,1",
                                    "7,0 -> 7,4",
                                    "6,4 -> 2,0",
                                    "0,9 -> 2,9",
                                    "3,4 -> 1,4",
                                    "0,0 -> 8,8",
                                    "5,5 -> 8,2", ],
                              true));
    }

    #[test]
    fn day05() {
        let mut file = File::open("data/day05.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let lines: Vec<&str> = buffer.lines().collect();

        assert_eq!(6311, _day05(&lines, false));
        assert_eq!(19929, _day05(&lines, true));
    }
}
