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

pub async fn assignment_4_b(input_number: &str) -> i64 {
    let lines: Vec<&str> = input_number
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();
    let mut grid: Vec<Vec<u8>> = lines.iter().map(|l| l.as_bytes().to_vec()).collect();

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
    loop {
        let mut changed = false;
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
                        && (grid[row_to_check as usize][column_to_check as usize] == b'@'
                            || grid[row_to_check as usize][column_to_check as usize] == b'X')
                    {
                        neighbors += 1;
                    }
                }

                if neighbors < 4 {
                    changed = true;
                    grid[row][column] = b'X';
                    count += 1;
                }
            }
        }
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == b'X' {
                    grid[r][c] = b'.';
                }
            }
        }
        if !changed {
            break;
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

    #[tokio::test]
    async fn test_example_4b() {
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
        assert_eq!(assignment_4_b(&input_number).await, 43);
    }
    #[tokio::test]
    async fn test_assignment_4_a() {
        let input = read_file("src/inputs/ass_04.txt");
        let expected: i64 = 1449;
        let result: i64 = assignment_4_a(&input).await; assert_eq!(result, expected);
    }
    #[tokio::test]
    async fn test_assignment_4_b() {
        let input = read_file("src/inputs/ass_04.txt");
        let expected: i64 = 8746;
        let result: i64 = assignment_4_b(&input).await; assert_eq!(result, expected);
    }
}
