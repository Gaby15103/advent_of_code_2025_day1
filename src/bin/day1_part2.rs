use std::io::{self, Read};

pub fn count_zero_hits_method_b(input: &str) -> usize {
    let mut pos = 50usize;
    let mut count = 0usize;

    for line in input
        .lines()
        .map(|l| l.trim_start_matches('\u{FEFF}').trim())
        .filter(|l| !l.is_empty())
    {
        let (dir, dist) = line.split_at(1);
        let distance: usize = dist.parse().unwrap();

        let step: i32 = match dir {
            "L" => -1,
            "R" => 1,
            _ => panic!("invalid direction"),
        };

        for _ in 0..distance {
            let p = pos as i32 + step;
            pos = (((p % 100) + 100) % 100) as usize; // safe modulo

            if pos == 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("failed to read input.txt");

    let result = count_zero_hits_method_b(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part2() {
        let example = r#"
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
"#;

        assert_eq!(count_zero_hits_method_b(example), 6);
    }
}
