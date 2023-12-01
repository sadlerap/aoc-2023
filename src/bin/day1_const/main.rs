use std::ops::RangeInclusive;

static PART1: u32 = part1();
static PART2: u32 = part2();

const fn part1() -> u32 {
    let mut input: &[u8] = include_bytes!("input.txt");
    let mut first = 0;
    let mut last = 0;
    let mut acc = 0;
    while let Some((next_char, remaining)) = input.split_first() {
        input = remaining;
        match next_char {
            b'\n' => {
                if first == 0 && last == 0 {
                    panic!("Invalid input!");
                }
                acc += 10 * first + last;
                first = 0;
                last = 0;
            }
            b'0'..=b'9' => {
                let c = (*next_char - b'0') as u32;
                if first == 0 && last == 0 {
                    first = c;
                    last = c;
                } else {
                    last = c;
                }
            }
            _ => {}
        }
    }

    acc
}

const fn part2() -> u32 {
    let mut input: &[u8] = include_bytes!("input.txt");
    let mut first = 0;
    let mut last = 0;
    let mut acc = 0;
    while let Some((next_char, remaining)) = input.split_first() {
        if *next_char == b'\n' {
            if first == 0 && last == 0 {
                panic!("Invalid input!");
            }
            acc += 10 * first + last;
            first = 0;
            last = 0;
        } else if let Some(x) = is_num_at_head(input) {
            if first == 0 {
                first = x;
            }
            last = x;
        }
        input = remaining;
    }
    acc
}

const fn range_contains(range: RangeInclusive<u8>, val: u8) -> bool {
    *range.start() <= val && val <= *range.end()
}

const fn is_num_at_head(input: &[u8]) -> Option<u32> {
    if input.is_empty() {
        return None;
    }

    if let Some(x) = input.first() {
        if range_contains(b'0'..=b'9', *x) {
            return Some((*x - b'0') as u32);
        }
    }

    if input.len() >= 3
        && input[0] == b'o'
        && input[1] == b'n'
        && input[2] == b'e' {
        return Some(1);
    }

    if input.len() >= 3 &&
        input[0] == b't' &&
        input[1] == b'w' &&
        input[2] == b'o' {
        return Some(2);
    }

    if input.len() >= 5
        && input[0] == b't'
        && input[1] == b'h'
        && input[2] == b'r'
        && input[3] == b'e'
        && input[4] == b'e'
    {
        return Some(3);
    }

    if input.len() >= 4
        && input[0] == b'f'
        && input[1] == b'o'
        && input[2] == b'u'
        && input[3] == b'r'
    {
        return Some(4);
    }

    if input.len() >= 4
        && input[0] == b'f'
        && input[1] == b'i'
        && input[2] == b'v'
        && input[3] == b'e'
    {
        return Some(5);
    }

    if input.len() >= 3 &&
        input[0] == b's' &&
        input[1] == b'i' &&
        input[2] == b'x' {
        return Some(6);
    }

    if input.len() >= 5
        && input[0] == b's'
        && input[1] == b'e'
        && input[2] == b'v'
        && input[3] == b'e'
        && input[4] == b'n'
    {
        return Some(7);
    }

    if input.len() >= 5
        && input[0] == b'e'
        && input[1] == b'i'
        && input[2] == b'g'
        && input[3] == b'h'
        && input[4] == b't'
    {
        return Some(8);
    }

    if input.len() >= 4
        && input[0] == b'n'
        && input[1] == b'i'
        && input[2] == b'n'
        && input[3] == b'e'
    {
        return Some(9);
    }

    None
}

fn main() {
    println!("{}: {}", "Part 1", PART1);
    println!("{}: {}", "Part 2", PART2);
}
