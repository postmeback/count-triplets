use std::io::{self, Write};

fn count_triplets(numbers: Vec<i32>, r: i32) -> usize {
    let n = numbers.len();
    let mut count = 0;

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if numbers[j] == numbers[i] * r && numbers[k] == numbers[j] * r {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    println!("Enter the numbers");

    let mut input_numbers = String::new();
    io::stdout().flush().unwrap(); // Flush the output buffer
    io::stdin().read_line(&mut input_numbers).expect("Invalid input");

    let num_arr: Vec<i32> = input_numbers
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid Input"))
        .collect();

    println!("Enter the common ratio");

    let mut common_ratio = String::new();
    io::stdin().read_line(&mut common_ratio).expect("Provide integer");

    let num_common_ratio: i32 = common_ratio.trim().parse().expect("Invalid Input");

    let result = count_triplets(num_arr, num_common_ratio);

    println!("The count is {}", result);
}
