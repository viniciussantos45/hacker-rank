use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plus_minus(arr: &[i32], len: &i32) {
    let mut positives: Vec<i32> = [].to_vec();
    let mut negatives: Vec<i32> = [].to_vec();
    let mut zeros: Vec<i32> = [].to_vec();

    for n in arr {
        if *n > 0 {
            positives.push(*n);
        } else if *n < 0 {
            negatives.push(*n);
        } else {
            zeros.push(*n);
        }
    }

    let positives_ratio = positives.len() as f32 / *len as f32;
    let negatives_ratio = negatives.len() as f32 / *len as f32;
    let zeros_ratio = zeros.len() as f32 / *len as f32;

    println!("{:.6}", positives_ratio);
    println!("{:.6}", negatives_ratio);
    println!("{:.6}", zeros_ratio);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plus_minus(&arr, &n);
}
