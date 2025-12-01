use log::{debug, info};
const START: i32 = 50;
const LIMIT: i32 = 100;

pub async fn assignment_1_a(input_list: &str) -> i32 {
    let mut number = START;
    let mut counter = 0;
    for item in parse_input(input_list) {
        debug!("old number: {}", number);
        number = (number + item).rem_euclid(LIMIT);
        debug!("new number: {}", number);
        if number == 0 {
            counter += 1;
            debug!("found one: {}", counter);
        }
    }
    counter
}

pub async fn assignment_1_b(input_list: &str) -> i32 {
    let mut number_moving = 50;
    let upper_limit = 100;
    let mut counter = 0;
    let list_items = parse_input(&input_list);
    for item in list_items {
        debug!("old number number:{} next item :{}", number_moving, item);
        let old_number_moving = number_moving;
        number_moving += item;
        if number_moving == 0 || number_moving == 100 {
            counter += 1;
            number_moving = 0;
            info!("found one:{}", counter,);
        } else if number_moving > upper_limit {
            let quotient = number_moving / upper_limit;
            let remainder = number_moving % upper_limit;
            counter += quotient.abs();
            number_moving = remainder;
            debug!(
                "remainder:{} quotient:{},new_number:{}, with new counter:{}",
                remainder, quotient, number_moving, counter
            );
        } else if number_moving < 0 {
            let quotient = number_moving / upper_limit;
            let remainder = number_moving % upper_limit;
            if old_number_moving != 0 {
                counter += quotient.abs() + 1;
            } else {
                counter += quotient.abs();
            }
            number_moving = if remainder == 0 {
                0
            } else {
                upper_limit + remainder
            };
            debug!(
                "remainder:{} quotient:{},new_number:{}, with new counter:{}",
                remainder,
                quotient.abs() + 1,
                remainder,
                counter
            );
        }
    }

    counter
}

fn parse_input(input_list: &str) -> Vec<i32> {
    input_list
        .lines()
        .map(|item| {
            let (dir, num) = item.split_at(1);
            let value: i32 = num.parse().expect("Invalid number");

            match dir {
                "L" => -value,
                "R" => value,
                _ => panic!("Invalid direction"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let input_string: String = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string();
        let converted_intput = vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];

        assert_eq!(parse_input(&input_string), converted_intput);
    }
    #[tokio::test]
    async fn test_1a() {
        let input_string: String = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string();

        assert_eq!(assignment_1_a(&input_string).await, 3)
    }

    #[tokio::test]
    async fn test_1b() {
        let _ = env_logger::builder()
            .is_test(true) // makes logs visible during `cargo test`
            .filter_level(log::LevelFilter::Info)
            .try_init();
        let input_string: String = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string();

        assert_eq!(assignment_1_b(&input_string).await, 6)
    }
}
