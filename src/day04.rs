use std::collections::HashSet;
use std::iter::FromIterator;

fn _parse(input: &str) -> (Vec<u64>, Vec<Vec<HashSet<u64>>>) {
    let lines: Vec<&str> = input.lines().collect();
    let drawn_numbers: Vec<u64> = lines[0].split(',')
        .map(|e| e.parse::<u64>().unwrap())
        .collect();

    let mut boards: Vec<Vec<HashSet<u64>>> = vec![];
    for board_index in (2..lines.len()).step_by(6) {
        let mut board: Vec<HashSet<u64>> = vec![];
        for row in 0..=4 {
            board.push(
                HashSet::from_iter(
                    lines[board_index + row].split_whitespace()
                        .map(|e| e.parse::<u64>().unwrap())
                )
            );
        }
        for col in 0..=4 {
            board.push(
                HashSet::from_iter(
                    (0..=4).map(|i|
                        lines[board_index + i].split_whitespace()
                            .collect::<Vec<&str>>()[col].parse().unwrap()
                    ),
                )
            )
        }
        boards.push(board);
    }
    (drawn_numbers, boards)
}

fn _day04_part1(input: &str) -> u64 {
    let (numbers, boards) = _parse(input);

    let mut drawn_numbers = HashSet::new();
    for number in numbers {
        drawn_numbers.insert(number);
        for board in boards.iter() {
            for row in board.iter() {
                if row.is_subset(&drawn_numbers) {
                    return board.iter().take(5)
                        .flat_map(|r| r.iter())
                        .filter(|&n| !drawn_numbers.contains(n))
                        .sum::<u64>() * number;
                }
            }
        }
    }

    panic!()
}

fn _day04_part2(input: &str) -> u64 {
    let (numbers, boards) = _parse(input);

    let mut drawn_numbers = HashSet::new();
    let mut left = boards.clone();
    for number in numbers {
        drawn_numbers.insert(number);
        let mut new_left: Vec<Vec<HashSet<u64>>> = vec![];
        for board in left.iter() {
            if !board.iter().any(|row| row.is_subset(&drawn_numbers)) {
                new_left.push(board.clone());
            }
        }
        if new_left.len() == 0 {
            return left.first().unwrap().iter().take(5)
                .flat_map(|r| r.iter())
                .filter(|&n| !drawn_numbers.contains(n))
                .sum::<u64>() * number;
        }
        left = new_left;
    }

    panic!()
}

#[cfg(test)]
mod tests_day04 {
    use std::fs::File;
    use std::io::Read;

    use crate::day04::{_day04_part1, _day04_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(4512, _day04_part1(&"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(1924, _day04_part2(&"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"));
    }

    #[test]
    fn day04() {
        let mut file = File::open("data/day04.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(58374, _day04_part1(&buffer));
        assert_eq!(11377, _day04_part2(&buffer));
    }
}
