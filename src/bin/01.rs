advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    const NEWLINE: u8 = '\n' as u8;

    let mut bytes = input.as_bytes().into_iter().copied();

    let mut state = 50;
    let mut count = 0;

    loop {
        let Some(char) = bytes.next() else {
            break;
        };
        let left = char == 'L' as u8;

        let mut digits = [0; 2];

        loop {
            match bytes.next().unwrap() {
                NEWLINE => break,
                char => digits = [(char - '0' as u8) as u64, digits[0]],
            };
        }
        let num = digits[0] + digits[1] * 10;

        state += if left { 100 - num } else { num };
        count += if state % 100 == 0 { 1 } else { 0 };
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    const NEWLINE: u8 = '\n' as u8;

    let mut bytes = input.as_bytes().into_iter().copied();

    let mut state = 50;
    let mut count = 0;

    loop {
        let Some(char) = bytes.next() else {
            break;
        };
        let left = char == 'L' as u8;

        let mut digits = [0; 3];

        loop {
            match bytes.next().unwrap() {
                NEWLINE => break,
                char => digits = [(char - '0' as u8) as u64, digits[0], digits[1]],
            };
        }

        if left {
            let low = digits[0] + digits[1] * 10;
            let high = digits[2];

            if state != 0 && state <= low {
                count += 1;
            }
            count += high;

            state = (state + (100 - low)) % 100;
        } else {
            let num = digits[0] + digits[1] * 10 + digits[2] * 100;

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
