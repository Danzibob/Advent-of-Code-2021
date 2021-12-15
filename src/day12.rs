use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> BTreeMap<i16,BTreeSet<i16>> {
    // First we need to assign numbers to each node
    let mut links: BTreeMap<i16,BTreeSet<i16>> = BTreeMap::new();
    let mut names: BTreeMap<&str, i16> = BTreeMap::new();
    names.insert("start", 0);
    names.insert("end", 1);
    links.insert(0, BTreeSet::new());
    links.insert(1, BTreeSet::new());
    let mut idx = 1;
    input.lines().for_each(|l| {
        let edge = l.split("-").map(|node| {
            // println!("Encountered node {}", node);
            if !names.contains_key(node){
                idx += 1;
                // Check if node is capital, sign based on this: pos NEG
                let name = if node.to_uppercase() == node {-idx} else {idx};
                names.insert(node, name);
                links.insert(name, BTreeSet::new());
                // println!("\tAdded new node record with val {}", idx);
                name
            } else {
                // println!("\tFound existing record with val {}", *names.get(node).unwrap());
                *names.get(node).unwrap()
            }
        }).collect::<Vec<i16>>();
        // println!("Mapping edge {:?}", edge);
        links.get_mut(&edge[0]).unwrap().insert(edge[1]);
        links.get_mut(&edge[1]).unwrap().insert(edge[0]);
    });
    links
}

fn count_paths(graph: &BTreeMap<i16,BTreeSet<i16>>, pos: i16, visited: &mut Vec<i16>) -> usize{
    let mut paths = 0;
    // println!("Depth: {} | Visited: {:?} | Pos: {}", visited.len(), visited, pos);
    for node in graph.get(&pos).unwrap() {
        if *node == 0 { continue }          // Start node
        if *node == 1 { paths += 1}         // End node
        else if !visited.contains(node) {   // Unvisited node
            if *node > 0 { visited.push(*node); }           // Small cave, log as visited
            paths += count_paths(graph, *node, visited);    // Recurse
            if *node > 0 { visited.pop(); }                 // Un-log to continue recursion
        }
    }
    paths
}

#[aoc(day12, part1)]
pub fn part1(input: &BTreeMap<i16,BTreeSet<i16>>) -> usize {
    count_paths(input, 0, &mut Vec::with_capacity(10)) // 0 is the start node
}

fn count_paths2(graph: &BTreeMap<i16,BTreeSet<i16>>, pos: i16, visited: &mut Vec<i16>, double_dipped: bool) -> usize{
    let mut paths = 0;
    // println!("Depth: {} | Visited: {:?} | Pos: {}", visited.len(), visited, pos);
    for node in graph.get(&pos).unwrap() {
        if *node == 0 { continue }          // Start node
        if *node == 1 { paths += 1}         // End node

        else if !visited.contains(node) {   // Unvisited node

            if *node > 0 { visited.push(*node); }  // Small cave, log as visited

            paths += count_paths2(graph, *node, visited, double_dipped);    // Recurse

            if *node > 0 { visited.pop(); }        // Un-log to continue recursion

        } else if (*node > 0) && !double_dipped {  // Visited small cave, but we haven't double dipped
            paths += count_paths2(graph, *node, visited, true);
        }
    }
    paths
}

#[aoc(day12, part2)]
pub fn part2(input: &BTreeMap<i16,BTreeSet<i16>>) -> usize {
    count_paths2(input, 0, &mut Vec::with_capacity(10), false) // 0 is the start node
}


#[cfg(test)]
mod tests {
    const TESTINPUT1: &str = 
"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

const TESTINPUT2: &str = 
"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    use super::*;
    #[test]
    fn day12_gen() {
        let inp = input_generator(TESTINPUT1);
        println!("{:?}", inp);
    }

    #[test]
    fn part1_test1() {
        let inp = input_generator(TESTINPUT1);
        // println!("hit");
        assert_eq!(part1(&inp), 10);
    }

    #[test]
    fn part1_test2() {
        let inp = input_generator(TESTINPUT2);
        // println!("hit");
        assert_eq!(part1(&inp), 226);
    }

    #[test]
    fn part2_test1() {
        let inp = input_generator(TESTINPUT1);
        // println!("hit");
        assert_eq!(part2(&inp), 36);
    }

    #[test]
    fn part2_test2() {
        let inp = input_generator(TESTINPUT2);
        // println!("hit");
        assert_eq!(part2(&inp), 3509);
    }
}