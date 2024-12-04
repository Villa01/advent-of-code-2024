advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut safe_reports = 0;

    for line in lines {
        let items = line.split_whitespace();

        let nums = items.map(|i| i.parse::<i32>().unwrap()).collect();

        if is_decreasing(&nums) || is_increasing(&nums) {
            safe_reports += 1;
        }
    }

    Some(safe_reports)
}

fn is_decreasing(nums: &Vec<i32>) -> bool {
    let mut nums_i = nums.iter().peekable();

    while let Some(c) = nums_i.next() {
        let next = nums_i.peek();
        // We can omit the last number to make the report valid.
        if next.is_none() {
            return true;
        }

        let next = *next.unwrap();
        if !(*c > *next && *next >= *c - 3) {
            return false;
        }
    }
    true
}

fn is_increasing(nums: &Vec<i32>) -> bool {
    let mut nums_i = nums.iter().peekable();

    while let Some(c) = nums_i.next() {
        let next = nums_i.peek();
        // We can omit the last number to make the report valid.
        if next.is_none() {
            return true;
        }

        let next = *next.unwrap();
        if !(*c < *next && *next <= *c + 3) {
            return false;
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut safe_reports = 0;

    for line in lines {
        let items = line.split_whitespace();

        let nums = items.map(|i| i.parse::<i32>().unwrap()).collect();

        let is_dec2 = is_decreasing2(&nums, false);
        let is_inc2 = is_increasing2(&nums, false);
        let is_safe = is_dec2 || is_inc2;

        if is_safe {
            safe_reports += 1;
        } else {
        }
    }

    Some(safe_reports)
}

fn is_decreasing2(nums: &Vec<i32>, already_omited: bool) -> bool {
    let mut nums_i = nums.iter().enumerate().peekable();

    while let Some((i, c)) = nums_i.next() {
        let next = nums_i.peek();
        // We can omit the last number to make the report valid.
        if next.is_none() {
            return true;
        }

        let (_, next) = *next.unwrap();
        if !(*c > *next && *next >= *c - 3) {
            if already_omited {
                return false;
            }

            //  We can only remove one number out of the sequence
            let mut skip_current = nums.clone();
            skip_current.remove(i);
            let omit_current = is_decreasing2(&skip_current, true);

            let mut skip_current = nums.clone();
            skip_current.remove(i + 1);
            let omit_next = is_decreasing2(&skip_current, true);
            return omit_current | omit_next;
        }
    }
    true
}

fn is_increasing2(nums: &Vec<i32>, already_omited: bool) -> bool {
    let mut nums_i = nums.iter().enumerate().peekable();

    while let Some((i, c)) = nums_i.next() {
        let next = nums_i.peek();
        // We can omit the last number to make the report valid.
        if next.is_none() {
            return true;
        }

        let (_, next) = *next.unwrap();
        if !(*c < *next && *next <= *c + 3) {
            if already_omited {
                return false;
            }

            let mut skip_current = nums.clone();
            skip_current.remove(i);

            let omit_current = is_increasing2(&skip_current, true);

            let mut skip_current = nums.clone();
            skip_current.remove(i + 1);

            let omit_next = is_increasing2(&skip_current, true);
            return omit_current | omit_next;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
