fn _day03_part1(input: &Vec<&str>) -> u64 {
    let counts: Vec<u64> = (0..input[0].len())
        .map(|index|
            input.iter()
                .map(|&item| if item.as_bytes()[index] as char == '1' { 1 } else { 0 })
                .sum()
        )
        .collect();

    let gamma: u64 = counts.iter()
        .map(|&count| if count > input.len() as u64 - count { 1 } else { 0 })
        .rev()
        .enumerate()
        .fold(0, |acc, (index, count)| { acc + count * 2_u64.pow(index as u32) });
    let epsilon: u64 = 2_i32.pow(input[0].len() as u32) as u64 - 1 - gamma;

    gamma * epsilon
}

fn _reduce_input(input: &Vec<&str>, f: fn(char) -> bool) -> String {
    let mut results = input.clone();

    let mut index = 0;
    while results.len() > 1 {
        let one_counts = results.iter()
            .filter(|&code| code.as_bytes()[index] as char == '1')
            .count();

        results = if one_counts >= results.len() - one_counts {
            results.into_iter()
                .filter(|&line| f(line.as_bytes()[index] as char))
                .collect()
        } else {
            results.into_iter()
                .filter(|&line| !f(line.as_bytes()[index] as char))
                .collect()
        };
        index += 1;
    }

    results.first().unwrap().to_string()
}

fn _day03_part2(input: &Vec<&str>) -> u64 {
    fn convert_result(result: &str) -> u64 {
        result.chars()
            .map(|c| if c == '1' { 1 } else { 0 })
            .rev()
            .enumerate()
            .fold(0, |acc, (index, count)| { acc + count * 2_u64.pow(index as u32) })
    }
    let n1 = convert_result(&_reduce_input(&input, |c| c == '1'));
    let n2 = convert_result(&_reduce_input(&input, |c| c == '0'));
    n1 * n2
}

#[cfg(test)]
mod tests_day03 {
    use std::fs::File;
    use std::io::Read;

    use crate::day03::{_day03_part1, _day03_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(198, _day03_part1(&vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(230, _day03_part2(&vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]));
    }

    #[test]
    fn day03() {
        let mut file = File::open("data/day03.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let lines: Vec<&str> = buffer.lines().collect();

        assert_eq!(2003336, _day03_part1(&lines));
        assert_eq!(1877139, _day03_part2(&lines));
    }
}
