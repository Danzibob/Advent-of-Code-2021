#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<String>{
    input.lines().map(|s| s.to_owned()).collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in input {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars(){
            let uxp = match c {
                '(' | '[' | '{' | '<' => {stack.push(c); 0},
                ')' => if stack.pop().unwrap() != '(' {3} else {0},
                ']' => if stack.pop().unwrap() != '[' {57} else {0},
                '}' => if stack.pop().unwrap() != '{' {1197} else {0},
                '>' => if stack.pop().unwrap() != '<' {25137} else {0}
                _ => {println!("Unexpected Character!"); 0}
            };
            // println!("Stack: {:?}", stack);
            // Check if corrupted
            sum += uxp;
        }
    }
    sum
}

fn closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => ' '
    }
}
fn score(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0
    }
}

#[aoc(day10, part2)]
pub fn part2(input: &Vec<String>) -> u64 {
    let mut scores: Vec<u64> = Vec::new();
    'line_loop: for line in input {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars(){
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                // Ignore corrupted
                _ => if closing(stack.pop().unwrap()) != c {
                    continue 'line_loop;
                }
            };
            
        }
        let score = stack.iter().enumerate().map(|(i, x)| {
            // println!("\t5^{} * {} '{}'", i, score(*x), *x);
            5_u64.pow(i as u32) * score(*x)
        }).sum();
        scores.push(score);
        // println!("Line: {} Score: {} Stack: {:?}", line, score, stack)
    }
    scores.sort();
    scores[scores.len() >> 1]
}

#[cfg(test)]
mod tests {
const TESTINPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    use super::*;
    #[test]
    fn part1_ex10() {
        let inp = input_generator(TESTINPUT);
        // println!("hit");
        assert_eq!(part1(&inp), 26397);
    }
    #[test]
    fn part2_ex10() {
        let inp = input_generator(TESTINPUT);
        // println!("hit");
        assert_eq!(part2(&inp), 288957);
    }
}