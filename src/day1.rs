#[aoc(day1, part1)]
fn part1(contents: &str) -> u32{
    let mut lines = contents.lines();
    let mut total_increases = 0;
    let mut last: u32 = lines.next().unwrap().parse().unwrap();
    let mut val: u32;
    for n in lines {
        val = n.parse().unwrap();
        if val > last{
            total_increases += 1;
        }
        last = val;
    }
    return total_increases;
}

#[aoc(day1, part2)]
fn part2(contents: &str) -> u32 {
    const WINDOW: usize = 3;
    let mut lines = contents.lines();
    let mut total_increases = 0;
    let mut last: [u32;WINDOW] = [0,0,0];
    for i in 0..WINDOW {
        last[i] = lines.next().unwrap().parse().unwrap();
    }
    for v in lines{
        let val = v.parse().unwrap();
        if last[0] < val {
            total_increases += 1;
        }
        for i in 0..(WINDOW-1) {
            last[i] = last[i+1];
        }
        last[WINDOW-1] = val;
    }
    return total_increases;
}