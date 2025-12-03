advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;

    for line in input.lines() {
        let mut max_digit = 0;
        let mut max_index = 0;

        for (i, digit) in line.as_bytes().into_iter().cloned().enumerate() {
            if digit > max_digit {
                max_digit = digit;
                max_index = i;
            }
        }

        if max_index == line.as_bytes().len() - 1 {
            let mut second_max_digit = 0;

            for digit in line.as_bytes()[0..max_index].into_iter().cloned() {
                if digit > second_max_digit {
                    second_max_digit = digit;
                }
            }

            sum += ((second_max_digit - '0' as u8) * 10 + (max_digit - '0' as u8)) as u64;
        } else {
            let mut second_max_digit = 0;

            for digit in line.as_bytes()[max_index + 1..].into_iter().cloned() {
                if digit > second_max_digit {
                    second_max_digit = digit;
                }
            }

            sum += ((max_digit - '0' as u8) * 10 + (second_max_digit - '0' as u8)) as u64;
        }
    }

    Some(sum)
}

fn get_next_digit(line: &str, used: &[usize]) -> usize {
    let mut max_num = 0;
    let mut max_index = 0;

    for j in 0..line.as_bytes().len() {
        if used.contains(&j) {
            continue;
        }

        let mut new_used = used.to_vec();
        new_used.push(j);

        let num = get_num(line, &new_used);

        if num > max_num {
            max_num = num;
            max_index = j;
        }
    }

    max_index
}

fn get_num(line: &str, used: &[usize]) -> u64 {
    let mut num = 0;

    for (i, c) in line.as_bytes().into_iter().cloned().enumerate() {
        if used.contains(&i) {
            num *= 10;
            num += (c - '0' as u8) as u64;
        }
    }

    num
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;

    for line in input.lines() {
        let mut digits = [0; 12];
        let mut indices = [0; 12];

        for i in 0..12 {
            let next_digit_index = get_next_digit(line, &indices[0..i]);
            indices[i] = next_digit_index;
            digits[i] = line.as_bytes()[next_digit_index];
        }

        let mut new_digits = [(0, 0); 12];
        for i in 0..12 {
            new_digits[i] = (digits[i], indices[i]);
        }
        new_digits.sort_by_key(|(_, i)| *i);

        let mut num = 0;
        for i in 0..12 {
            num *= 10;
            num += (new_digits[i].0 - '0' as u8) as u64;
        }

        sum += num;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
