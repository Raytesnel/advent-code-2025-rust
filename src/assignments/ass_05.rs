use log::{debug};
use crate::utils::read_file;


pub async fn assignment_5_b(input: &str) -> i64 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut intervals: Vec<(i64, i64)> = Vec::new();

    for line in parts[0].lines() {
        let (s, e) = line.split_once('-').unwrap();
        intervals.push((s.parse().unwrap(), e.parse().unwrap()));
    }
    intervals.sort_by_key(|&(s, _)| s);
    let mut merged: Vec<(i64, i64)> = Vec::new();
    for (start, end) in intervals {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }
    merged
        .into_iter()
        .map(|(s, e)| e - s + 1)
        .sum()
}


pub async fn assignment_5_a(input: &str) -> i64 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    debug!("Parts: {:?}", parts);
    let range_lines: Vec<&str> = parts[0].lines().collect();
    let mut number_lines: Vec<i64> = parts[1]
        .lines()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let mut counter: i64 = 0;

    for r in range_lines {
        let (start_s, end_s) = r.split_once('-').unwrap();
        let start: i64 = start_s.parse().unwrap();
        let end: i64 = end_s.parse().unwrap();

        let mut remaining = Vec::new();

        for &num in &number_lines {
            if num >= start && num <= end {
                counter += 1;
                debug!("Number {} is in range {}-{}", num, start, end);
            } else {
                remaining.push(num);
            }
        }

        number_lines = remaining;
    }

    counter
}




#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_add() {
        let input_string: String = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#.to_string();

        assert_eq!(assignment_5_a(&input_string).await, 3);
    }
    #[tokio::test]
    async fn test_assignment_5_a() {
        let input = read_file("src/inputs/ass_05.txt");
        let expected: i64 = 640;
        let result: i64 = assignment_5_a(&input).await;
        assert_eq!(result, expected);
    }
    #[tokio::test]
    async fn test_sample_5b() {
        let input_string: String = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#.to_string();

        assert_eq!(assignment_5_b(&input_string).await, 14);
    }
    
    #[tokio::test]
    async fn test_assignment_5_b() {
        let input = read_file("src/inputs/ass_05.txt");
        let expected: i64 = 365804144481581;
        let result: i64 = assignment_5_b(&input).await;
        assert_eq!(result, expected);
    }
    
}