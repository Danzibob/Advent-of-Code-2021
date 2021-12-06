pub struct Problem {
    numbers: Vec<u8>,
    boards: Vec<[[u8; 5]; 5]>
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Problem {
    let mut lines = input.lines();
    // Read first line and parse as u8
    let numbers = lines.next().unwrap()
                    .split(",")
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect();
    // Create empty Boards vector
    let mut boards: Vec<[[u8; 5]; 5]> = Vec::new();
    // While loop consumes empty line
    while lines.next() != None {
        let mut b: [[u8; 5]; 5] = [[0; 5]; 5];
        for y in 0..5 {
            let l = lines.next().unwrap();
            for x in 0..5 {
                b[y][x] = (&l[(x*3)..(x*3+2)]).chars()
                            .filter(|c| *c != ' ').collect::<String>()
                            .parse::<u8>().unwrap();
            }
        }
        boards.push(b);
    }

    // //println!("{:?}", boards.pop().unwrap());
    return Problem{numbers, boards};
}

#[aoc(day4, part1)]
fn part1(input: &Problem) ->  u32{
    // Re-order numbers to a dict of order called
    let mut called: [u8; 100] = [0; 100];
    for (i, n) in (&input.numbers).iter().enumerate() {
        called[*n as usize] = i as u8;
    }

    let mut min_board = 100;
    let mut best_board = 0;
    let mut this_board = 0;
    
    for b in &input.boards {
        // Map board to new ordering system
        let mapped = (*b).map(|row| row.map(|x| called[x as usize]));
        //println!("pre : {:?}\npost : {:?}", (*b), mapped);
        
        let mut min_line = 100;

        // Check each row for a minimum value
        for row in mapped {
            let mx = row.iter().max().unwrap();
            // //println!("row mx: {}", mx);
            if *mx < min_line {
                min_line = *mx;
            }
        }

        // Check each column for a min value
        for i in 0..5 {
            let col = mapped.map(|x| x[i]);
            let mx = col.iter().max().unwrap();
            // //println!("col mx: {}", mx);
            if *mx < min_line {
                min_line = *mx;
            }
        }

        //println!("Best line: {}", min_line);

        // This max is the last number called on the card
        // Now we can update the best card
        if min_line < min_board {
            min_board = min_line;
            best_board = this_board;
        }
        this_board += 1;
    }

    //println!("Best Board: {}", best_board);

    // Now we have the winning board and the last number called
    // so we can now collect all the unmarked numbers#
    let mapped = (&input.boards)[best_board].map(|row| row.map(|x| called[x as usize]));
    let mut uncalled_sum: u32 = 0;
    for row in mapped {
        //println!("{:?}", row);
        for item in row {
            if item > min_board {
                let call_index = item;
                uncalled_sum += input.numbers[call_index as usize] as u32;
                //println!("{} -> {}", call_index, input.numbers[call_index as usize]);
            }
        }
    }

    //println!("Sum of uncalled = {}", uncalled_sum);

    let last_called = input.numbers[min_board as usize];

    //println!("Last called = {} -> {}", min_board, last_called);
    
    return (last_called as u32) * uncalled_sum;
}

#[aoc(day4, part2)]
fn part2(input: &Problem) ->  u32{
    // Re-order numbers to a dict of order called
    let mut called: [u8; 100] = [0; 100];
    for (i, n) in (&input.numbers).iter().enumerate() {
        called[*n as usize] = i as u8;
    }

    let mut max_board = 0;
    let mut best_board = 0;
    let mut this_board = 0;
    
    for b in &input.boards {
        // Map board to new ordering system
        let mapped = (*b).map(|row| row.map(|x| called[x as usize]));
        //println!("pre : {:?}\npost : {:?}", (*b), mapped);
        
        let mut min_line = 100;

        // Check each row for a minimum value
        for row in mapped {
            let mx = row.iter().max().unwrap();
            // //println!("row mx: {}", mx);
            if *mx < min_line {
                min_line = *mx;
            }
        }

        // Check each column for a min value
        for i in 0..5 {
            let col = mapped.map(|x| x[i]);
            let mx = col.iter().max().unwrap();
            // //println!("col mx: {}", mx);
            if *mx < min_line {
                min_line = *mx;
            }
        }

        //println!("Worst line: {}", min_line);

        // This max is the last number called on the card
        // Now we can update the best card
        if min_line > max_board {
            max_board = min_line;
            best_board = this_board;
        }
        this_board += 1;
    }

    //println!("Best Board: {}", best_board);

    // Now we have the winning board and the last number called
    // so we can now collect all the unmarked numbers#
    let mapped = (&input.boards)[best_board].map(|row| row.map(|x| called[x as usize]));
    let mut uncalled_sum: u32 = 0;
    for row in mapped {
        //println!("{:?}", row);
        for item in row {
            if item > max_board {
                let call_index = item;
                uncalled_sum += input.numbers[call_index as usize] as u32;
                //println!("{} -> {}", call_index, input.numbers[call_index as usize]);
            }
        }
    }

    //println!("Sum of uncalled = {}", uncalled_sum);

    let last_called = input.numbers[max_board as usize];

    //println!("Last called = {} -> {}", max_board, last_called);
    
    return (last_called as u32) * uncalled_sum;
}




#[allow(dead_code)]
const TESTINPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example() {
        let inp = input_generator(TESTINPUT);
        assert_eq!(part1(&inp), 4512);
    }
    #[test]
    fn part2_example() {
        let inp = input_generator(TESTINPUT);
        assert_eq!(part2(&inp), 1924);
    }
}