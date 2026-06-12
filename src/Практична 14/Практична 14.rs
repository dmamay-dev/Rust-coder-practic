use std::io::{self, BufRead};

fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    let k = k as usize;
    let total_shared_sum: i32 = bill
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != k)
        .map(|(_, &price)| price)
        .sum();

    let anna_actual_share = total_shared_sum / 2;

    if b == anna_actual_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b - anna_actual_share);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let _n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let bill: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    bonAppetit(&bill, k, b);
}