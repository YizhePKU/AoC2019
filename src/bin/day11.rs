use aoc2019::IntcodeComputer;
use std::collections::HashMap;

fn main() {
    let mut pc = IntcodeComputer::from_file("data/day11");
    let mut paint = HashMap::new();
    let mut pos = (0, 0);
    let mut dir = (-1, 0);
    while !pc.halted {
        pc.run();

        if let Some(color) = pc.output.pop_front() {
            paint.insert(pos, color);

            let turn = pc.output.pop_front().unwrap();
            if turn == 1 {
                dir = (dir.1, -dir.0);
            } else {
                dir = (-dir.1, dir.0);
            }
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }

        if !pc.halted {
            let value = paint.get(&pos).copied().unwrap_or_default();
            pc.input.push_back(value);
        }
    }
    dbg!(paint.len());

    let mut pc = IntcodeComputer::from_file("data/day11");
    let mut paint = HashMap::new();
    // start the paint on a white panel
    paint.insert((0, 0), 1);
    let mut pos = (0, 0);
    let mut dir = (-1, 0);
    while !pc.halted {
        pc.run();

        if let Some(color) = pc.output.pop_front() {
            paint.insert(pos, color);

            let turn = pc.output.pop_front().unwrap();
            if turn == 1 {
                dir = (dir.1, -dir.0);
            } else {
                dir = (-dir.1, dir.0);
            }
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }

        if !pc.halted {
            let value = paint.get(&pos).copied().unwrap_or_default();
            pc.input.push_back(value);
        }
    }
    
    // render the paint
    let x_min = paint.keys().map(|&(x, _)| x).min().unwrap();
    let x_max = paint.keys().map(|&(x, _)| x).max().unwrap();
    let y_min = paint.keys().map(|&(_, y)| y).min().unwrap();
    let y_max = paint.keys().map(|&(_, y)| y).max().unwrap();
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let color = paint.get(&(x, y)).copied().unwrap_or_default();
            if color == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
