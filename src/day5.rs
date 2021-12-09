use std::convert::TryInto;
use std::cmp;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<[u32; 4]> {
    let mut problem: Vec<[u32; 4]> = Vec::new();
    for l in input.lines(){
        let parts: [u32; 4] = l.split(" -> ")
                                .map(|w| w.split(",")
                                          .map(|x| x.parse::<u32>().unwrap())
                                          .collect::<Vec<u32>>()
                                )
                                .flatten().collect::<Vec<u32>>()
                                .try_into().unwrap();
        problem.push(parts);
    }
    return problem;
}

#[aoc(day5, part1)]
fn part1(input: &Vec<[u32; 4]>) ->  u32{
    // println!("Hit 2");
    let mut board = vec![vec![0_u32; 1000]; 1000];

    // println!("{:?}", board);

    for line in input {
        // Check if line is horizontal or vertical
        if line[0] == line[2] {
            // println!("Y-iter {:?}", line);
            // x coords are the same
            let mx = cmp::max(line[1], line[3]);
            let mn = cmp::min(line[1], line[3]);
            for y in mn..=mx {
                board[y as usize][line[0] as usize] += 1;
            }
        } else if line[1] == line[3] {
            // println!("X-iter {:?}", line);
            // y coords are the same
            let mx = cmp::max(line[0], line[2]);
            let mn = cmp::min(line[0], line[2]);
            for x in mn..=mx {
                board[line[1] as usize][x as usize] += 1;
            }
        } else {
            // println!("Skipping {:?}", line);
        }

        // println!("{:?}", board);
    }

    return board.iter().flatten().filter(|i| **i > 1).count() as u32;
}

fn grad(l:  [u32; 4]) -> (i32, i32) {
    let dx = (l[0] as i32) - (l[2] as i32);
    let dy = (l[1] as i32) - (l[3] as i32);
    return (dx, dy)
}

fn is_diag(l: [u32; 4]) -> bool{
    let (dx, dy) = grad(l);
    return dx.abs() == dy.abs();
}

#[aoc(day5, part2)]
fn part2(input: &[[u32; 4]]) ->  u32{
    // println!("Hit 2");
    let mut board = vec![vec![0_u32; 1000]; 1000];

    // println!("{:?}", board);

    for line in input {
        // Check if line is horizontal or vertical
        // println!("");
        if line[0] == line[2] {
            // println!("Y-iter {:?}", line);
            // x coords are the same
            let mx = cmp::max(line[1], line[3]);
            let mn = cmp::min(line[1], line[3]);
            for y in mn..=mx {
                board[y as usize][line[0] as usize] += 1;
            }
        } else if line[1] == line[3] {
            // println!("X-iter {:?}", line);
            // y coords are the same
            let mx = cmp::max(line[0], line[2]);
            let mn = cmp::min(line[0], line[2]);
            for x in mn..=mx {
                board[line[1] as usize][x as usize] += 1;
            }
        } else if is_diag(*line){
            // println!("Diag-iter {:?}", line);

            let mx_x = cmp::max(line[0], line[2]);
            let mn_x = cmp::min(line[0], line[2]);
            let mx_y = cmp::max(line[1], line[3]);
            let mn_y = cmp::min(line[1], line[3]);

            let (dx, dy) = grad(*line);
            if dx*dy > 0 {
                // Gradient is positive 
                for (x, y) in (mn_x..=mx_x).zip(mn_y..=mx_y){
                    board[y as usize][x as usize] += 1;
                }
            } else {
                for (x, y) in (mn_x..=mx_x).zip((mn_y..=mx_y).rev()){
                    board[y as usize][x as usize] += 1;
                }
            }
        } else {
            // println!("Skipping {:?}", line);
        }

        // println!("{:?}", board);
    }

    return board.iter().flatten().filter(|i| **i > 1).count() as u32;
}


#[cfg(test)]
mod tests {

    const TESTINPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    use super::*;
    #[test]
    fn test_gen(){
        assert_eq!(input_generator(TESTINPUT), [[0, 9, 5, 9], [8, 0, 0, 8], [9, 4, 3, 4], [2, 2, 2, 1], [7, 0, 7, 4], [6, 4, 2, 0], [0, 9, 2, 9], [3, 4, 1, 4], [0, 0, 8, 8], [5, 5, 8, 2]]);
    }
    #[test]
    fn part1_day5() {
        let inp = input_generator(TESTINPUT);
        println!("hit");
        assert_eq!(part1(&inp), 5);
    }
    #[test]
    fn part2_example() {
        let inp = input_generator(TESTINPUT);
        assert_eq!(part2(&inp), 12);
    }
}