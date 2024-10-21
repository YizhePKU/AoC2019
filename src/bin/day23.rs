use aoc2019::IntcodeComputer;
use std::collections::HashSet;

fn main() {
    let computer = IntcodeComputer::from_file("data/day23");

    // Boot up 50 computers.
    let mut computers = vec![computer; 50];
    for i in 0..50 {
        computers[i].input.push_back(i as i64);
    }

    let mut nat_history = HashSet::new();
    loop {
        // Run all computers until they block on input.
        for i in 0..50 {
            computers[i].run();
        }

        // Collect all the packets.
        let mut packets = vec![vec![]; 50];
        let mut nat = (0, 0);
        for i in 0..50 {
            while let Some(addr) = computers[i].output.pop_front() {
                let x = computers[i].output.pop_front().unwrap();
                let y = computers[i].output.pop_front().unwrap();
                if addr == 255 {
                    nat = (x, y);
                } else {
                    packets[addr as usize].push((x, y));
                }
            }
        }

        // If the network is idle, send NAT packet to address 0.
        if packets.iter().all(|packet| packet.is_empty()) {
            packets[0].push(nat);

            if nat_history.contains(&nat) {
                println!("part2 = {}", nat.1);
                return;
            } else {
                nat_history.insert(nat);
            }
        }

        // Deliver all the packets.
        for (i, packet) in packets.into_iter().enumerate() {
            if packet.is_empty() {
                computers[i].input.push_back(-1);
            } else {
                for (x, y) in packet {
                    computers[i].input.push_back(x);
                    computers[i].input.push_back(y);
                }
            }
        }
    }
}
