advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut bytes = input.as_bytes().into_iter().copied();

    let mut state = 50;
    let mut count = 0;

    loop {
        let mut num = 0;

        let Some(char) = bytes.next() else {
            break;
        };
        let left = char == 'L' as u8;

        loop {
            let char = bytes.next().unwrap();
            const NEWLINE: u8 = '\n' as u8;

            match char {
                NEWLINE => break,
                _ => num = num * 10 + (char - '0' as u8) as u64,
            };
        }

        state += if left { 100 - num % 100 } else { num };
        count += if state % 100 == 0 { 1 } else { 0 };
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut bytes = input.as_bytes().into_iter().copied();

    let mut state = 50;
    let mut count = 0;

    loop {
        let mut num = 0;

        let Some(char) = bytes.next() else {
            break;
        };
        let left = char == 'L' as u8;

        loop {
            let char = bytes.next().unwrap();
            const NEWLINE: u8 = '\n' as u8;

            match char {
                NEWLINE => break,
                _ => num = num * 10 + (char - '0' as u8) as u64,
            };
        }

        if left {
            if state != 0 && state <= num % 100 {
                count += 1;
            }
            count += num / 100;

            state = (state + (100 - num % 100)) % 100;
        } else {
            count += (state + num) / 100;

            state = (state + num) % 100;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
