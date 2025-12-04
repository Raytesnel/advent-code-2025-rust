pub async fn assignment_4_a(input_number: &str) -> i64 {
    let lines: Vec<&str> = input_number
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();
    let grid: Vec<&[u8]> = lines.iter().map(|l| l.as_bytes()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;

    for row in 0..rows {
        for column in 0..cols {
            if grid[row][column] != b'@' {
                continue;
            }

            let mut neighbors = 0;

            for (direct_row, direct_column) in dirs {
                let row_to_check = row as isize + direct_row;
                let column_to_check = column as isize + direct_column;

                if row_to_check >= 0
                    && row_to_check < rows as isize
                    && column_to_check >= 0
                    && column_to_check < cols as isize
                    && grid[row_to_check as usize][column_to_check as usize] == b'@'
                {
                    neighbors += 1;
                }
            }

            if neighbors < 4 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_example_4a() {
        let input_number = r#"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@."#;
        assert_eq!(assignment_4_a(&input_number).await, 13);
    }
}
