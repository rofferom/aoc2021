use std::fs::File;
use std::io::{BufRead, BufReader};

struct Window {
    sum: u32,
    count: u8,
}

fn main() {
    let mut count = 0;
    let mut prev: Option<u32> = None;
    let mut windows: Vec<Window> = vec![];

    let file = File::open("src/day1/input.txt").unwrap();

    for line in BufReader::new(file).lines() {
        let value: u32 = line.unwrap().parse().unwrap();

        // Update existing Windows
        for win in &mut windows {
            win.sum += value;
            win.count += 1;
        }

        // Add a new Window
        windows.push(Window {
            sum: value,
            count: 1,
        });

        // Pull first item if required
        if windows[0].count == 3 {
            let win = windows.remove(0);

            if let Some(prev) = prev {
                if win.sum > prev {
                    count += 1;
                }
            }

            prev = Some(win.sum);
        }
    }

    println!("{}", count);
}
