use aoc::utils::get_lines;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Pair {
    x: i32,
    y: i32,
    distance: i32,
}

impl Eq for Pair {}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Pair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn recordWireLocations(wireLocations: &mut HashSet<Pair>, movements: &Vec<String>) {
    let mut distance: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for movement in movements.iter() {
        let direction = match movement.chars().next().unwrap() {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!("wtf"),
        };

        let magnitude = movement[1..].parse::<i32>().unwrap();
        if direction == Direction::Left || direction == Direction::Right {
            let sign = match direction {
                Direction::Left => -1,
                Direction::Right => 1,
                _ => unreachable!(),
            };

            for i in 0..magnitude {
                distance = distance + 1;
                x = x + sign;
                wireLocations.insert(Pair { x, y, distance });
            }
        } else {
            let sign = match direction {
                Direction::Down => -1,
                Direction::Up => 1,
                _ => unreachable!(),
            };

            for i in 0..magnitude {
                distance = distance + 1;
                y = y + sign;
                wireLocations.insert(Pair { x, y, distance });
            }
        }
    }
}

fn main() {
    let paths = get_lines("input.txt");
    let firstWireMoves: Vec<String> = paths[0].split(',').map(|x| x.to_owned()).collect();
    let secondWireMoves: Vec<String> = paths[1].split(',').map(|x| x.to_owned()).collect();

    let mut firstWireLocations: HashSet<Pair> = HashSet::new();
    let mut secondWireLocations: HashSet<Pair> = HashSet::new();

    recordWireLocations(&mut firstWireLocations, &firstWireMoves);
    recordWireLocations(&mut secondWireLocations, &secondWireMoves);

    let intersection: HashSet<&Pair> = firstWireLocations
        .intersection(&secondWireLocations)
        .collect();

    // Part 1
    {
        let mut minDistance = -1;
        for pair in intersection.iter() {
            let distance = pair.x.abs() + pair.y.abs();
            if distance < minDistance || minDistance == -1 {
                minDistance = distance;
            }
        }

        println!("{:?}", minDistance);
    }

    // Part 2
    {
        let mut minSumOfDistances = -1;
        for pair in intersection.iter() {
            let sumOfDistances = firstWireLocations.get(pair).unwrap().distance
                + secondWireLocations.get(pair).unwrap().distance;
            if sumOfDistances < minSumOfDistances || minSumOfDistances == -1 {
                minSumOfDistances = sumOfDistances;
            }
        }

        println!("{:?}", minSumOfDistances);
    }
}
