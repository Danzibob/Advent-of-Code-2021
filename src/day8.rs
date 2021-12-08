#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input.lines().map(|line| {
        let mut parts = line.split(" | ");
        let patterns = parts.next().unwrap();
        let output = parts.next().unwrap();
        (
            patterns.split(" ").map(|s| s.to_owned()).collect(),
            output.split(" ").map(|s| s.to_owned()).collect()
        )
    }).collect()
}

#[aoc(day8, part1)]
fn part1(input: &Vec<(Vec<String>, Vec<String>)>) -> u32{
    let counts = input.iter().map(|l| &l.1)
                    .flatten()
                    .fold(vec![0; 7], |mut count, digit| {
                        count[digit.len()-1] += 1;
                        count
                    });
    // println!("Digit size counts: {:?}", counts);
    return (counts[1] + counts[2] + counts[3] + counts[6]) as u32;
}

#[aoc(day8, part2)]
fn part2(input: &Vec<(Vec<String>, Vec<String>)>) -> u32{
    let mut sum = 0;
    for line in input {
        // With reference to the example drawing
        // the two letters in 1 are       c/f - O
        // the remaining letter in 7 is   a   - S
        // the remaining letters in 4 are b/d - F
        // the remaining letters in 8 are e/g - E
        // 0 - OOSFEE
        // 1 - known
        // 2 - OSFEE
        // 3 - OOSFE
        // 4 - known
        // 5 - OSFFE
        // 6 - OSFFEE
        // 7 - known
        // 8 - known
        // 9 - OOSFFE
        // Decision tree:
        // Len 5:
        //      2 Os -> 3
        //          2 Fs -> 5
        //          else -> 2
        // else:
        //      2 Os:
        //          2 Fs -> 9
        //          else -> 0
        //      else -> 6

        // println!("");

        // Locate the one and four characters
        let mut one = line.0.iter().find(|x| x.len() == 2).unwrap().chars();
        let onea = one.next().unwrap();
        let oneb = one.next().unwrap();
        let mut four = line.0.iter().find(|x| x.len() == 4).unwrap().chars()
                        .filter(|x| *x != onea && *x != oneb);
        let foura = four.next().unwrap();
        let fourb = four.next().unwrap();

        // println!("One: {}{}, Four: {}{}", 
                // onea, oneb, foura, fourb);

        // For each word, get the length and count of F and O characters
        // (Would probably have been faster to replace all the letters at once 
        //  but too late now heheh)
        let decoded = line.1.iter().map(|word| {
            let ones = word.chars().filter(|c| *c == onea || *c == oneb).count();
            let fours = word.chars().filter(|c| *c == foura || *c == fourb).count();
            let length = word.chars().count();
            // println!("{}", word);
            // println!("Ones: {} Fours: {} Chars: {}", ones, fours, length);
            if length == 2 { 1 }
            else if length == 3 { 7 }
            else if length == 4 { 4 }
            else if length == 7 { 8 }
            else if length == 5 {
                if ones == 2 { 3 }
                else {
                    if fours == 2 { 5 }
                    else { 2 }
                }
            } else {
                if ones == 1 { 6 }
                else {
                    if fours == 2 { 9 }
                    else { 0 }
                }
            }
        });

        // println!("{:?}", decoded.collect::<Vec<usize>>());
        
        sum += decoded.rev().enumerate().fold(0, |tot, (i, digit)| {
            // println!("digit:{}, i:{}, result:{}", digit, i, (usize::pow(10, i as u32) * digit));
            tot + (usize::pow(10, i as u32) * digit)
        } )
    }
    return sum as u32;
}

#[cfg(test)]
mod tests {

    const TESTINPUT: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    use super::*;
    #[test]
    fn part1_ex() {
        let inp = input_generator(TESTINPUT);
        // println!("hit");
        assert_eq!(part1(&inp), 2);
    }
    #[test]
    fn part2_ex() {
        let inp = input_generator(TESTINPUT);
        // println!("hit");
        assert_eq!(part2(&inp), 5353);
    }
}