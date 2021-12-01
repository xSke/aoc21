use aoc21::*;

fn count_increased(i: impl Iterator<Item=usize>) -> usize {
    // lol procedural
    let mut last_depth = None;
    let mut count = 0;
    for depth in i {
        if let Some(last_depth) = last_depth {
            if depth > last_depth {
                count += 1;
            }
        }

        last_depth = Some(depth);
    }

    count
}

fn main() {
    let depths = input_lines_parsed::<usize>(1);

    let part1 = count_increased(depths.iter().cloned());
    println!("part 1: {}", part1);

    let windows = depths.windows(3).map(|x| x.iter().sum());
    let part2 = count_increased(windows);
    println!("part 2: {}", part2);
}
