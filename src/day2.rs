#[aoc(day2, part1)]
fn part1(input: &str) ->  u32{
    let (mut horiz, mut vert) = (0,0);
    for l in input.lines() {
        let mut command = l.split(" ");
        let word = command.next().unwrap();
        let value: u32 = command.next().unwrap().parse().unwrap();
        match word {
            "forward" => horiz += value,
            "up" => vert -= value,
            "down" => vert += value,
            _ => panic!()
        };
    }

    return horiz * vert;
}

#[aoc(day2, part2)]
fn part2(input: &str) ->  u32{
    let (mut horiz, mut vert, mut aim) = (0,0,0);
    for l in input.lines() {
        let mut command = l.split(" ");
        let word = command.next().unwrap();
        let value: u32 = command.next().unwrap().parse().unwrap();
        match word.chars().next().unwrap() {
            'f' => {
                horiz += value;
                vert += aim * value;
            },
            'u' => aim -= value,
            'd' => aim += value,
            _ => panic!()
        };
    }

    return horiz * vert;
}