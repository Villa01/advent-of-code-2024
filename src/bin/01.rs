use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first_list, mut second_list) = parse_input(input);

    assert_eq!(first_list.len(), second_list.len());

    first_list.sort_unstable();
    second_list.sort_unstable();

    let mut differences: Vec<i32> = Vec::new();
    for (i, item) in first_list.iter().enumerate() {
        let diff = (item - second_list[i]).abs();
        differences.push(diff);
    }
    let sum: i32 = differences.iter().sum();
    Some(sum as u32)
}

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    let lines = input.split("\n");
    lines.for_each(|line| {
        let v = line.split(" ").collect::<Vec<&str>>();
        if v.len() > 3 {
            let item = *v.first().unwrap();
            let n: i32 = item.parse().unwrap();
            first_list.push(n);

            let item = *v.last().unwrap();
            let n: i32 = item.parse().unwrap();
            second_list.push(n);
        }
    });

    return (first_list, second_list);
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first_list, second_list) = parse_input(input);

    assert_eq!(first_list.len(), second_list.len());

    let mut occurences: HashMap<i32, usize> = HashMap::new();

    for n in second_list.iter() {
        match occurences.get(n) {
            Some(v) => {
                occurences.insert(*n, v + 1);
            }
            None => {
                occurences.insert(*n, 1);
            }
        }
    }

    let mut similarity_scores: Vec<i32> = Vec::new();
    for item in first_list.iter() {
        let sim_score = item * (*occurences.get(item).unwrap_or(&0) as i32);
        similarity_scores.push(sim_score);
    }
    let sum: i32 = similarity_scores.iter().sum();
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
