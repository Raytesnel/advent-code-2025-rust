use env_logger;
use std::fs;
use std::time::Instant;

mod assignments {
    pub mod ass_01;
}

fn main() {
    let inputs = [(
        "src/inputs/ass_01.txt",
        vec![
            assignments::ass_01::assigment_1_a,
            assignments::ass_01::assigment_1_b,
        ],
    )];

    for (file_path, assignments) in inputs {
        let contents = read_file(file_path);

        for (i, assignment) in assignments.iter().enumerate() {
            let now = Instant::now();
            let result = assignment(&contents);
            let elapsed = now.elapsed();
            println!(
                "Result for {:<25} part-{:?}: {:<15} (Time elapsed: {:>8.2?}µs)",
                file_path,
                i + 1,
                result,
                elapsed.as_micros()
            );
        }
    }
}

fn read_file(input_path: &str) -> String {
    fs::read_to_string(input_path).expect("LogRocket: Should have been able to read the file{}")
}
