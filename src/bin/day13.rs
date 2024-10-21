use aoc2019::IntcodeComputer;
use std::{collections::HashMap, time::Duration};

fn print_screen(screen: &HashMap<(i64, i64), i64>) {
    let tiles = [' ', 'H', 'X', '=', 'o'];

    let x_min = screen.keys().map(|k| k.0).min().unwrap();
    let x_max = screen.keys().map(|k| k.0).max().unwrap();
    let y_min = screen.keys().map(|k| k.1).min().unwrap();
    let y_max = screen.keys().map(|k| k.1).max().unwrap();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let id = screen[&(x, y)];
            print!("{}", tiles[id as usize]);
        }
        println!();
    }
}

fn main() {
    let mut pc = IntcodeComputer::from_file("data/day13");
    let mut screen = HashMap::new();
    pc.run();
    while let Some(x) = pc.output.pop_front() {
        let y = pc.output.pop_front().unwrap();
        let tile = pc.output.pop_front().unwrap();
        screen.insert((x, y), tile);
    }
    let block_tile_count = screen.values().filter(|&&v| v == 2).count();
    dbg!(block_tile_count);

    let mut pc = IntcodeComputer::from_file("data/day13");
    let mut screen = HashMap::new();
    let mut score = 0;
    let mut ball = None;
    let mut paddle = None;
    pc.mem[0] = 2;
    while !pc.halted {
        // Run the program and examine the output.
        pc.run();
        while let Some(x) = pc.output.pop_front() {
            let y = pc.output.pop_front().unwrap();
            let tile = pc.output.pop_front().unwrap();
            if x == -1 {
                score = tile;
            } else {
                screen.insert((x, y), tile);

                if tile == 3 {
                    paddle = Some((x, y));
                }
                if tile == 4 {
                    ball = Some((x, y));
                }
            }
        }

        // Print the screen and the current score.
        println!("------------------------------------------");
        println!("score = {score}");
        println!();
        print_screen(&screen);
        println!();

        // Pause a bit.
        std::thread::sleep(Duration::from_millis(100));

        // Provide input to the program.
        if paddle.unwrap().0 > ball.unwrap().0 {
            pc.input.push_back(-1);
        } else if paddle.unwrap().0 < ball.unwrap().0 {
            pc.input.push_back(1);
        } else {
            pc.input.push_back(0);
        }
    }
}
