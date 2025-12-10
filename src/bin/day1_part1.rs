use std::io::{self, Read};

fn count_zero_hits(input: &str) -> usize {
    let mut pos: i32 = 50;
    let mut zeros = 0usize;

    for raw in input.lines() {
        let mut line = raw.trim();
        line = line.trim_start_matches('\u{feff}'); // remove BOM

        if line.is_empty() {
            continue;
        }

        let (dir, num_str) = line.split_at(1);
        let dist: i32 = num_str.trim().parse().expect("invalid number");

        match dir {
            "L" | "l" => pos -= dist,
            "R" | "r" => pos += dist,
            _ => panic!("invalid rotation"),
        }

        pos = pos.rem_euclid(100);
        if pos == 0 {
            zeros += 1;
        }
    }

    zeros
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("failed to read input.txt");

    let result = count_zero_hits(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(count_zero_hits(input), 3);
    }
}