const WIDTH: usize = 50;
const HEIGHT: usize = 10;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (Vec<(usize, usize)>, Vec<(bool, usize)>){
    let mut points = Vec::with_capacity(1024);
    let mut folds = Vec::with_capacity(16);
    let mut lines = input.lines();

    // Get the points
    loop {
        let line = lines.next().unwrap();
        if line == "" { break }
        let mut coords = line.split(",").map(|x| x.parse::<usize>().unwrap());
        points.push((coords.next().unwrap(), coords.next().unwrap()));
    }

    // Get the folds
    for line in lines {
        let mut parts = line.split("=");
        let axis = parts.next().unwrap().chars().last().unwrap();
        let value = parts.next().unwrap().parse::<usize>().unwrap();
        folds.push((axis == 'x', value));
    }

    (points, folds)
}

#[aoc(day13, part1)]
pub fn part1(input: &(Vec<(usize, usize)>, Vec<(bool, usize)>)) -> usize {
    let (points, folds) = input;
    let (axis, line) = folds[0];
    points.len() - 
    if axis { // Fold line is x = n
        points.iter().filter(|(x,y)| *x > line && points.contains(&(line+line-*x, *y))).count()
    } else {  // Fold line is y = n
        points.iter().filter(|(x,y)| *y > line && points.contains(&(*x, line+line-*y))).count()
    }
}

#[aoc(day13, part2)]
pub fn part2(input: &(Vec<(usize, usize)>, Vec<(bool, usize)>)) -> u32 {
    let (points, folds) = input;
    let mut pts: Vec<(usize, usize)> = points.iter().map(|x| *x).collect(); // a shitty copy
    for (axis, line) in folds{
        if *axis { // Fold line is x = n
            pts = pts.iter().map(|(x,y)| if x > line {(*line + *line-*x, *y)} else {(*x, *y)}).collect();
        } else {  // Fold line is y = n
            pts = pts.iter().map(|(x,y)| if y > line {(*x, *line + *line-*y)} else {(*x, *y)}).collect();
        }
    }
    for y in 0..HEIGHT {
        let mut s = String::with_capacity(WIDTH*2);
        for x in 0..WIDTH {
            s.push_str(if pts.contains(&(x,y)) {"##"} else {"  "});
        }
        println!("{}", s);
    }

    0
}

#[cfg(test)]
mod tests {
    const TESTINPUT: &str = 
"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    use super::*;
    #[test]
    fn generator() {
        let inp = input_generator(TESTINPUT);
        println!("{:?}", inp);
    }

    #[test]
    fn part1_test() {
        let inp = input_generator(TESTINPUT);
        assert_eq!(part1(&inp), 17);
    }

    #[test]
    fn part2_test() {
        let inp = input_generator(TESTINPUT);
        assert_eq!(part2(&inp), 0);
    }
}