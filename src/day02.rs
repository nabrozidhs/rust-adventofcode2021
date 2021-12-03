use crate::util::Vector2;

fn _parse(line: &str) -> Vector2 {
    let split: Vec<&str> = line.split_whitespace().collect();
    let distance: i64 = split[1].parse().unwrap();
    match split[0] {
        "forward" => Vector2::new(distance, 0),
        "up" => Vector2::new(0, -distance),
        "down" => Vector2::new(0, distance),
        _ => panic!()
    }
}

fn _day02_part1(input: &Vec<&str>) -> i64 {
    let result = input.iter()
        .map(|e| _parse(*e))
        .fold(Vector2::ZERO, |acc, e| acc + e);

    result.0 * result.1
}

fn _day02_part2(input: &Vec<&str>) -> i64 {
    let (horizontal, depth, _) = input.iter()
        .map(|e| _parse(*e))
        .fold((0, 0, 0), |(horizontal, depth, aim), e| {
            if e.1 == 0 {
                (horizontal + e.0, depth + e.0 * aim, aim)
            } else {
                (horizontal, depth, aim + e.1)
            }
        });

    horizontal * depth
}

#[cfg(test)]
mod tests_day02 {
    use std::fs::File;
    use std::io::Read;

    use crate::day02::{_day02_part1, _day02_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(150, _day02_part1(&vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"]));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(900, _day02_part2(&vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"]));
    }

    #[test]
    fn day02() {
        let mut file = File::open("data/day02.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let lines: Vec<&str> = buffer.lines().collect();

        assert_eq!(2036120, _day02_part1(&lines));
        assert_eq!(2015547716, _day02_part2(&lines));
    }
}
