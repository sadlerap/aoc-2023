use color_eyre::eyre::{bail, Context, Result};

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input)?);
    println!("Part 2: {}", part2(input)?);

    Ok(())
}

fn is_num(c: u8) -> Option<u32> {
    if (b'0'..=b'9').contains(&c) {
        Some((c - b'0') as u32)
    } else {
        None
    }
}

fn part1(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| {
            let Some((first_digit, index)) = line
                .bytes()
                .enumerate()
                .find_map(|(index, c)| is_num(c).map(|r| (r, index)))
            else {
                bail!("Failed to find first digit in record {}", line)
            };
            let last_digit = line
                .bytes()
                .skip(index)
                .rev()
                .find_map(is_num)
                .unwrap_or(first_digit);
            Ok(first_digit * 10 + last_digit)
        })
        .try_fold(0, |x, y| Ok(x + y?))
}

static DIGITS: [(&str, u32); 18] = [
    ("one", 1),
    ("1", 1),
    ("two", 2),
    ("2", 2),
    ("three", 3),
    ("3", 3),
    ("four", 4),
    ("4", 4),
    ("five", 5),
    ("5", 5),
    ("six", 6),
    ("6", 6),
    ("seven", 7),
    ("7", 7),
    ("eight", 8),
    ("8", 8),
    ("nine", 9),
    ("9", 9),
];

fn starts_with_digit(input: &str) -> Option<u32> {
    DIGITS.iter().find_map(|(name, digit)| {
        if input.starts_with(name) {
            Some(*digit)
        } else {
            None
        }
    })
}

fn parse_first_last(line: &str) -> Result<u32> {
    let Some((first, index)) =
        (0..line.len()).find_map(|i| starts_with_digit(&line[i..]).map(|x| (x, i)))
    else {
        bail!("Failed to find first digit in record {}", line)
    };

    let last = (index..line.len())
        .rev()
        .find_map(|i| starts_with_digit(&line[i..]))
        .unwrap_or(first);

    Ok(first * 10 + last)
}

fn part2(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| parse_first_last(line).wrap_err("Failed to parse line"))
        .try_fold(0, |x, y| Ok(x + y?))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(part1(input)?, 142);
        Ok(())
    }

    #[test]
    fn test_part_1_solution() -> Result<()> {
        let input = include_str!("input.txt");
        assert_eq!(part1(input)?, 54630);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(part2(input)?, 281);
        Ok(())
    }

    #[test]
    fn test_part_2_solution() -> Result<()> {
        let input = include_str!("input.txt");
        assert_eq!(part2(input)?, 54770);
        Ok(())
    }
}
