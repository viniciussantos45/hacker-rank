use std::io::{self, BufRead};

/*
 * Complete the 'mini_max_sum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn mini_max_sum(arr: &[i32]) {
    let mut arr = arr.to_vec();

    arr.sort();

    let min_sum: i64 = arr.iter().take(4).map(|&x| x as i64).sum();
    let max_sum: i64 = arr.iter().skip(1).map(|&x| x as i64).sum();

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    mini_max_sum(&arr);
}
