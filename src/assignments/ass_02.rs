use log::{debug, info};

pub async fn assignment_2_a(text_input: &str) -> i64 {
    let mut list_of_numbers = vec![];
    for range in text_input.split(',') {
        debug!("Range: {:?}", range);
        let range = range.split('-').collect::<Vec<&str>>();
        let start = range[0].parse::<i64>().unwrap();
        let end = range[1].parse::<i64>().unwrap();
        debug!("Start: {}, End: {}", start, end);
        list_of_numbers.push(find_doubles(start, end));
    }
    let summed =  list_of_numbers.iter().flatten().sum();
    info!("Summed: {:?}", summed);
    summed
}

pub async fn assignment_2_b(text_input: &str) -> i64 {
    let mut list_of_numbers = vec![];
    for range in text_input.split(',') {
        debug!("Range: {:?}", range);
        let range = range.split('-').collect::<Vec<&str>>();
        let start = range[0].parse::<i64>().unwrap();
        let end = range[1].parse::<i64>().unwrap();
        debug!("Start: {}, End: {}", start, end);
        list_of_numbers.push(find_doubles_2(start, end));
    }
    let summed =  list_of_numbers.iter().flatten().sum();
    info!("Summed: {:?}", summed);
    summed
}

fn check_for_sillynes(input: &str) -> bool {
    if input.len() % 2 != 0 {
        return false;
    }

    let mid = input.len() / 2;
    input[0..mid] == input[mid..]
}

fn check_for_sillynes_2(input: &str) -> bool {
    let n = input.len();
    for d in 1..=n / 2 {
        if n % d != 0 {
            continue;
        }
        let chunk = &input[0..d];
        let repeats = n / d;
        if chunk.repeat(repeats) == input {
            return true;
        }
    }

    false
}
fn find_doubles_2(start_range: i64, end_range: i64) -> Vec<i64> {
    let mut doubles = vec![];
    for number in start_range..=end_range {
        if check_for_sillynes_2(&number.to_string()) {
            doubles.push(number);
        }
    }
    doubles
}

fn find_doubles(start_range: i64, end_range: i64) -> Vec<i64> {
    let mut doubles = vec![];
    for number in start_range..=end_range {
        if check_for_sillynes(&number.to_string()) {
            doubles.push(number);
        }
    }
    doubles
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_doubles() {
        let start = 11;
        let end = 22;
        let expected = vec![11, 22];
        let result = find_doubles(start, end);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_doubles_2() {
        let start = 38593856;
        let end = 38593862;
        let expected = vec![38593859];
        let result = find_doubles(start, end);
        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn test_example() {
        let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(assignment_2_a(&test_input).await, 1227775554);
    }    
    #[tokio::test]
    async fn test_example_2() {
        let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(assignment_2_b(&test_input).await, 4174379265);
    }
}
