use std::collections::BTreeSet;

const PUZZLECOLS: usize = 10;
const PUZZLEROWS: usize = 5;
const PUZZLESIZE: usize = PUZZLECOLS * PUZZLEROWS;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> [u32; PUZZLESIZE]{
    let mut arr = [0; PUZZLESIZE];
    let mut i: usize = 0;
    for c in input.chars() {
        if c == '\n' {continue}
        arr[i] = c.to_digit(10).unwrap();
        i += 1;
    }
    return arr;
}

#[aoc(day9, part1)]
pub fn part1(input: &[u32; PUZZLESIZE]) -> u32 {
    let mut sum = 0;
    for i in 0..PUZZLESIZE {
        let conditions = vec![
            // Y-1
            if i >= PUZZLECOLS {input[i-PUZZLECOLS]} else {10},
            // X-1
            if i%PUZZLECOLS > 0 {input[i-1]} else {10},
            // X+1
            if i%PUZZLECOLS < PUZZLECOLS-1 {input[i+1]} else {10},
            // Y+1
            if i+PUZZLECOLS < PUZZLESIZE {input[i+PUZZLECOLS]} else {10}
        ];
        if conditions.iter().all(|x| *x > input[i]) {
            sum += input[i] + 1;
        }
    }
    return sum;
}

#[aoc(day9, part2)]
pub fn part2(input: &[u32; PUZZLESIZE]) -> u32 {
    // Find a list of all the low points
    // These are where we'll start our traversal from
    let mut low_points: Vec<usize> = Vec::new();
    for i in 0..PUZZLESIZE {
        let conditions = vec![
            // Y-1
            if i >= PUZZLECOLS {input[i-PUZZLECOLS]} else {10},
            // X-1
            if i%PUZZLECOLS > 0 {input[i-1]} else {10},
            // X+1
            if i%PUZZLECOLS < PUZZLECOLS-1 {input[i+1]} else {10},
            // Y+1
            if i+PUZZLECOLS < PUZZLESIZE {input[i+PUZZLECOLS]} else {10}
        ];
        if conditions.iter().all(|x| *x > input[i]) {
            low_points.push(i);
        }
    }

    let mut basin_sizes: Vec<usize> = Vec::new();

    for point in low_points{
        // println!("\nExploring low point {}", point);
        let mut explored: BTreeSet<usize> = BTreeSet::new();
        let mut unexplored: BTreeSet<usize> = BTreeSet::new();
        unexplored.insert(point);
        while unexplored.len() > 0{
            let i = *unexplored.iter().next().unwrap();
            unexplored.remove(&i);
            explored.insert(i);
            let neighbors = vec![
                // Y-1
                if i >= PUZZLECOLS {i-PUZZLECOLS} else {PUZZLESIZE},
                // X-1
                if i%PUZZLECOLS > 0 {i-1} else {PUZZLESIZE},
                // X+1
                if i%PUZZLECOLS < PUZZLECOLS-1 {i+1} else {PUZZLESIZE},
                // Y+1
                if i+PUZZLECOLS < PUZZLESIZE {i+PUZZLECOLS} else {PUZZLESIZE}
            ];
            // println!("\tNeighbors to {}: {:?}", i, neighbors);
            let _new = neighbors.iter().filter(|x| {
                        **x != PUZZLESIZE
                        && input[**x] != 9
                        && !explored.contains(*x)
                    }).map(|x| unexplored.insert(*x)).count();
            // println!("\tAdded {} new points", new);
        }
        basin_sizes.push(explored.len())
    }

    basin_sizes.sort();
    return basin_sizes.iter().rev().take(3).product::<usize>() as u32;
}

#[cfg(test)]
mod tests {
const TESTINPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    use super::*;
    #[test]
    fn test_gen(){
        let inp = input_generator(TESTINPUT);
        // println!("{:?}", inp);
        assert_eq!(inp, [2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9, 2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8]);
    }
    #[test]
    fn part1_ex9() {
        let inp = input_generator(TESTINPUT);
        // println!("hit");
        assert_eq!(part1(&inp), 2);
    }
    #[test]
    fn part2_ex9() {
        let inp = input_generator(TESTINPUT);
        // println!("hit");
        assert_eq!(part2(&inp), 1134);
    }
}