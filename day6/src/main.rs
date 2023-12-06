const RACES: [(usize, usize); 4] = [(0, 0), (0, 0), (0, 0), (0, 0)];

fn part1() {
    let mut wins: [usize; 4] = [0; 4];
    for (idx, (time, distance)) in RACES.into_iter().enumerate() {
        for i in 0..time {
            let speed = i;
            let duration = time - i;
            if speed * duration > distance {
                wins[idx] += 1;
            }
        }
    }
    println!("part1: {}", wins.into_iter().product::<usize>())
}

const RACE: (usize, usize) = (0, 0);

fn part2() {
    let mut wins = 0;
    let (time, distance) = RACE;
    for i in 0..time {
        let speed = i;
        let duration = time - i;
        if speed * duration > distance {
            wins += 1;
        }
    }

    println!("part2: {}", wins)
}

fn main() {
    part1();
    part2();
}
