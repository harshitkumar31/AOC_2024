use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Expected input");

    let contents_arr: Vec<&str> = contents.split("\n").collect();
    let mut result = 0;
    for line in contents_arr {
        let line_arr: Vec<&str> = line.split(" ").collect();
        let mut prev = -1;
        let mut prevTrend = 0; // 1 is increasing, -1 is decreasing
        let mut safe = true;
        for num in line_arr {
            let num = num.parse::<i32>().unwrap();
            if prev == -1 {
                prev = num;
                continue;
            }
            let diff = num.abs_diff(prev);
            if !(diff >= 1 && diff <= 3) {
                safe = false;
                break;
            }
            if num > prev {
                if prevTrend == -1 {
                    safe = false;
                    break;
                } else {
                    prevTrend = 1;
                }
            }
            if num < prev {
                if prevTrend == 1 {
                    safe = false;
                    break;
                } else {
                    prevTrend = -1;
                }
            }

            prev = num;
        }
        if safe {
            result += 1;
        }
    }
    println!("Result is {result}");
}
