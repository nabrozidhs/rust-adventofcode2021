use itertools::Itertools;

fn _day01_part1(input: &Vec<u64>) -> usize {
    input.iter().tuple_windows::<(_,_)>()
        .filter(|(left, right)| right > left)
        .count()
}

fn _day01_part2(input: &Vec<u64>) -> usize {
    input.iter().tuple_windows::<(_,_,_)>().map(|e| e.0 + e.1 + e.2).tuple_windows::<(_,_)>()
        .filter(|(left, right)| right > left)
        .count()
}

#[cfg(test)]
mod tests_day01 {
    use std::fs::File;
    use std::io::Read;

    use crate::day01::{_day01_part1,_day01_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(7, _day01_part1(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(5, _day01_part2(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]));
    }

    #[test]
    fn day01() {
        let mut file = File::open("data/day01.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let input: Vec<u64> = buffer.lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect();

        assert_eq!(1722, _day01_part1(&input));
        assert_eq!(1748, _day01_part2(&input));
    }
}
