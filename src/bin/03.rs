use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"(?m)mul\((?<n1>[0-9]{1,3}),(?<n2>[0-9]{1,3})\)").unwrap();

    let nums: Vec<(i32, i32)> = regex
        .captures_iter(input)
        .map(|c| {
            let n1 = c
                .name("n1")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .expect("Should be a number.");
            let n2 = c
                .name("n2")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .expect("Should be a number.");
            (n1, n2)
        })
        .collect();

    let result: i32 = nums.iter().map(|(n1, n2)| n1 * n2).sum();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Horrible regex that captures mul(X,Y), do(), or don't
    let regex =
        Regex::new(r"(?m)((?<mul>mul\((?<n1>([0-9]{1,3})),(?<n2>[0-9]{1,3})\))|(?<dont>don't\(\))|(?<do>do()))")
            .unwrap();

    let mut allow_multiply = true;

    let result: i32 = regex
        .captures_iter(input)
        .map(|c| {
            if let Some(_mul) = c.name("mul") {
                let n1 = c
                    .name("n1")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .expect("Must be a number");

                let n2 = c
                    .name("n2")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .expect("Must be a number");

                return if allow_multiply { n1 * n2 } else { 0 };
            } else if let Some(_) = c.name("dont") {
                allow_multiply = false;
            } else if let Some(_) = c.name("do") {
                allow_multiply = true;
            }

            return 0;
        })
        .sum();

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
