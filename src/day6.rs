#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> [u64; 9] {
    let mut counts = [0; 9];
    for idx in input.split(","){
        counts[idx.parse::<usize>().unwrap()] += 1
    }
    return counts;
}

#[aoc(day6, part1)]
fn part1(input: &[u64; 9]) ->  u64{
    let mut today = *input;
    // println!("{:?}", today);
    let mut tmp0: u64;
    for _ in 0..80 {
        tmp0 = today[0];
        for idx in 1..=8 {
            today[idx-1] = today[idx];
        }
        today[8] = tmp0;
        today[6] += tmp0;
        // println!("{:?}", today);
    }
    return today.iter().sum();
}

#[aoc(day6, part2)]
fn part2(input: &[u64; 9]) ->  u64{
    let mut today = *input;
    // println!("{:?}", today);
    let mut tmp0: u64;
    for _ in 0..256 {
        tmp0 = today[0];
        for idx in 1..=8 {
            today[idx-1] = today[idx];
        }
        today[8] = tmp0;
        today[6] += tmp0;
        // println!("{:?}", today);
    }
    return today.iter().sum();
}

#[cfg(test)]
mod tests {

    const TESTINPUT: &str = "3,4,3,1,2";
    use super::*;
    #[test]
    fn test_gen(){
        let inp = input_generator(TESTINPUT);
        // println!("{:?}", inp);
        assert_eq!(inp, [0,1,1,2,1,0,0,0,0]);
    }
    #[test]
    fn part1_example() {
        let inp = input_generator(TESTINPUT);
        // println!("hit");
        assert_eq!(part1(&inp), 5934_u64);
    }
    #[test]
    fn part2_example() {
        let inp = input_generator(TESTINPUT);
        assert_eq!(part2(&inp), 26984457539_u64);
    }
}