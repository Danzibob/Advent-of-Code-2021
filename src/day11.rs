#![allow(dead_code)]
use std::char;

const WIDTH: usize = 10;
const SIZE: usize = 100;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<u32>{
    let out = input.lines().map(|s|
        s.chars().map(|c| c.to_digit(10).unwrap())
    ).flatten().collect();
    out
}

fn surround(i: usize) -> Vec<usize>{
    let mut n = Vec::new();
    let col = i%WIDTH;
    let row = i/WIDTH;
    if col != 0 {
        n.push(i-1);
        if row > 0 {n.push(i-WIDTH-1)}
        if row < WIDTH-1 {n.push(i+WIDTH-1)}
    }
    if col != 9 {
        n.push(i+1);
        if row > 0 {n.push(i-WIDTH+1)}
        if row < WIDTH-1 {n.push(i+WIDTH+1)}
    }
    if row > 0 {n.push(i-WIDTH)}
    if row < WIDTH-1 {n.push(i+WIDTH)}
    // println!("\tNeighbors for {}: {:?}", i, n);
    n
}

fn grid(input: &Vec<u32>) -> String {
    input.iter().map(|x| char::from_digit(*x, 36).unwrap())
                .collect::<Vec<char>>()
                .chunks(WIDTH)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
}

fn step(mut input: Vec<u32>) -> Vec<u32>{
    let mut add: Vec<u32> = vec![1;100];
    let mut new_add: Vec<u32> = vec![0;100];
    while {
        let mut done = false;
        // println!("Input:\n{}", grid(&input));
        // println!("Add:\n{}", grid(&add));
        // combine the add window and find pops
        for ((i, x), a) in input.iter_mut().enumerate().zip(&add){
            if *x > 9 {continue} // Already popped this cell
            *x = *x + a;
            if *x > 9 {
                done = true;
                // Add the popped increments to the new_add array
                // println!("\tPopped {}", i);
                for p in surround(i){
                    new_add[p] += 1;
                }
            }
        }
        
        add = new_add.iter().cloned().collect();
        new_add = vec![0; 100];
        // While condition
        done
    }{}
    return input.iter().map(|x| if *x>9 {0} else {*x}).collect();
}

#[aoc(day11, part1)]
pub fn part1(input: &Vec<u32>) -> u32 {
    let mut flashes = 0;
    let mut input = input.clone();
    for _ in 0..100 {
        input = step(input);
        flashes += input.iter().filter(|x| **x == 0).count();
    }
    flashes as u32
}

#[aoc(day11, part2)]
pub fn part2(input: &Vec<u32>) -> u32 {
    let mut steps = 0;
    let mut input = input.clone();
    loop{
        input = step(input);
        steps += 1;
        if input.iter().all(|x| *x == 0) {break}
    }
    steps
}

#[cfg(test)]
mod tests {
    const TESTINPUT: &str = 
"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    use super::*;
    #[test]
    fn part1_ex11() {
        let inp = input_generator(TESTINPUT);
        // println!("hit");
        assert_eq!(part1(&inp), 1656);
    }
    #[test]
    fn part2_ex10() {
        let inp = input_generator(TESTINPUT);
        // println!("hit");
        assert_eq!(part2(&inp), 195);
    }
}