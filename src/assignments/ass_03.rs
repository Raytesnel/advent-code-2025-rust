pub async fn assignment_3_a(input_list: &str) -> u64 {
    input_list.lines().into_iter().map(|line| find_biggest_number(line, 2)).collect::<Vec<u64>>().iter().sum()
}

pub async fn assignment_3_b(input_list: &str) -> u64 {
    input_list.lines().into_iter().map(|line| find_biggest_number(line, 12)).collect::<Vec<u64>>().iter().sum()
}

pub fn find_biggest_number(input: &str, number_length:usize) -> u64 {
    if input.len() < number_length {
        return 0;
    }

    let chars: Vec<char> = input.chars().collect();
    let mut result = Vec::with_capacity(number_length);

    let mut start_index = 0;

    for k in 0..number_length {
        let remaining = number_length - k;
        let max_search_end = chars.len() - (remaining - 1);

        let mut best_digit = '0';
        let mut best_i = start_index;

        for i in start_index..max_search_end {
            let c = chars[i];
            if c > best_digit {
                best_digit = c;
                best_i = i;
            }
        }

        result.push(best_digit);
        start_index = best_i + 1;
    }

    result.into_iter().collect::<String>().parse().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_3a_single() {
        let input_number = "818181911112111";
        assert_eq!(find_biggest_number(&input_number, 2), 92);
    }
    #[test]
    fn test_example_3b_single() {
        let input_number= "818181911112111";
        assert_eq!(find_biggest_number(&input_number, 12), 888911112111);
    }
    
    #[tokio::test]
    async fn test_example_3a() {
        let input_number = r#"
            987654321111111
            811111111111119
            234234234234278
            818181911112111"#;
        assert_eq!(assignment_3_a(&input_number).await, 357);
    }    
    #[tokio::test]
    async fn test_example_3b() {
        let input_number = r#"
            987654321111111
            811111111111119
            234234234234278
            818181911112111"#;
        assert_eq!(assignment_3_b(&input_number).await, 3121910778619);
    }
    #[tokio::test]
    async fn test_assignment_3_a() {
        let input = read_file("src/inputs/ass_03.txt");
        let expected: u64 = 17301;
        let result: u64 = assignment_3_a(&input).await; assert_eq!(result, expected);
    }
    #[tokio::test]
    async fn test_assignment_3_b() {
        let input = read_file("src/inputs/ass_03.txt");
        let expected: u64 = 172162399742349;
        let result: u64 = assignment_3_b(&input).await; assert_eq!(result, expected);
    }
}