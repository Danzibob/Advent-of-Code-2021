const WIDTH: usize = 12;

#[aoc(day3, part1)]
fn part1(input: &str) ->  u32{
    // Array to track total digit occurances 
    // (1 -> +1, 0 -> -1)
    let mut gamma: [i32; WIDTH] = [0; WIDTH];
    for l in input.lines() {
        for (i, c) in l.chars().enumerate() {
            gamma[i] += if c == '0' {-1} else {1};
        }
    }

    // Convert positive values to their relevant binary 
    // values and sum these together
    let mut sum = 0;
    for (digit, val) in gamma.iter().enumerate(){
        sum += ((*val > 0) as u32) << (WIDTH - digit - 1);
    }

    // XOR with 0xFFF to NOT with a window of 12 bits
    return sum * (0xFFF ^ sum);
}

// Returns the most common digit in the position
fn count_digit(list: &Vec<u32>, pos: usize) -> i32 {
    let mut count = 0;
    for val in list {
        // println!("{} && {} = {}", val, 1 << pos, (1 << pos) & val);
        if ((1 << pos) & val) != 0 {
            count += 1;
        } else {
            count -= 1;
        }
    }
    return count;
}

#[aoc(day3, part2)]
fn part2(input: &str) ->  u32{
    // Parse input into vector of uints 
    let lines: Vec<u32> = input.lines().map(|x| u32::from_str_radix(x, 2).unwrap()).collect();
    
    let mut pool = lines.clone();
    let mut o2gen = 0;
    for d in (0..12).rev(){
        let digit = if count_digit(&pool, d) >= 0 {1} else {0};
        pool = pool.into_iter().filter(|x| ((x >> d) ^ digit)&1 == 0).collect();
        if pool.iter().count() == 1 {
            o2gen = pool[0];
            break;
        }
    }
    // println!("O2 Generator Rating {} ({:012b})", o2gen, o2gen);

    pool = lines.clone();
    let mut co2scrub = 0;
    for d in (0..12).rev(){
        let digit = if count_digit(&pool, d) >= 0 {0} else {1};
        // println!("Digit: {} -> {}", count_digit(&pool, d), digit);
        pool = pool.into_iter().filter(|x| ((x >> d) ^ digit)&1 == 0).collect();
        // println!("Remaining: {}", pool.iter().count());
        if pool.iter().count() == 1 {
            co2scrub = pool[0];
            break;
        }
    }
    // println!("CO2 Scrubber Rating {} ({:012b})", co2scrub, co2scrub);

    return o2gen*co2scrub;
}
