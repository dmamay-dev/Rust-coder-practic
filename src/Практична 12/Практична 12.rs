use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let mut max_val = 0;
    let mut count = 0;

    for &candle in candles {
        if candle > max_val {
            max_val = candle;
            count = 1;
        } else if candle == max_val {
            count += 1;
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _candles_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let candles: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = birthdayCakeCandles(&candles);

    writeln!(&mut fptr, "{}", result).ok();
}