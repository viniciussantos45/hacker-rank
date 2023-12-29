use std::io::{self, BufRead};

/*
 * Complete the 'time_conversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn time_conversion(s: &str) -> String {
    match &s.chars().rev().take(2).collect::<String>()[..]
        .chars()
        .rev()
        .collect::<String>()[..]
    {
        "PM" => {
            let mut hour = s.chars().take(2).collect::<String>().parse::<u8>().unwrap();
            if hour != 12 {
                hour += 12;
            }
            format!("{:02}{}", hour, &s[2..s.len() - 2])
        }
        "AM" => {
            let mut hour = s.chars().take(2).collect::<String>().parse::<u8>().unwrap();
            if hour == 12 {
                hour = 0;
            }
            format!("{:02}{}", hour, &s[2..s.len() - 2])
        }
        _ => {
            panic!("Invalid time format")
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = time_conversion(&s);

    println!("{}", result);
}
