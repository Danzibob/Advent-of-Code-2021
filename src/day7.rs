#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<u32> {
    return input.split(",").map(|x| x.parse().unwrap()).collect();
}

#[aoc(day7, part1)]
fn part1(input: &Vec<u32>) -> u32{
    let mut best_fuel = 10000000;
    for i in 0..1000{
        let fuel = input.iter().fold(0, |total, item| total + (i - (*item as i32)).abs() as u32);
        if fuel < best_fuel {
            best_fuel = fuel;
        }
    }
    return best_fuel as u32;
}


#[cfg(test)]
mod tests {

    const TESTINPUT: &str = "16,1,2,0,4,2,7,1,2,14";
    use super::*;
    #[test]
    fn test_gen(){
        let inp = input_generator(TESTINPUT);
        // println!("{:?}", inp);
        assert_eq!(inp, [3,4,3,1,2]);
    }
    #[test]
    fn part1_ex() {
        let inp = input_generator(TESTINPUT);
        // println!("hit");
        assert_eq!(part1(&inp), 37);
    }
    // #[test]
    // fn part2_example() {
    //     let inp = input_generator(TESTINPUT);
    //     assert_eq!(part2(&inp), 26984457539_u64);
    // }
}