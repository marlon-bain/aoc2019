use aoc::intcode::IntCodeMachine;
use aoc::utils::get_lines;
use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone, Debug)]
enum Color {
    White,
    Black,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn paint_with_starting_tile(starting_tile: Color, registers: &Vec<i64>) {
    let mut painted: HashMap<Coordinate, Color> = HashMap::new();
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut facing_direction: Direction = Direction::Up;
    let mut current_coordinate = Coordinate { x: 0, y: 0 };

    let mut machine = IntCodeMachine::new(&registers);
    let mut iterations = 0;
    loop {
        iterations += 1;
        let current_color: Color = match painted.get(&current_coordinate) {
            Some(c) => c.clone(),
            None => starting_tile.clone(),
        };

        let color_input: i64 = match current_color {
            Color::White => 1,
            Color::Black => 0,
        };

        machine.provide_input(color_input);

        let color_output: i64 = match machine.get_output() {
            None => break,
            Some(v) => v,
        };

        let new_color: Color = match color_output {
            1 => Color::White,
            0 => Color::Black,
            _ => panic!(),
        };

        painted.insert(current_coordinate.clone(), new_color);

        let should_turn_left: bool = match machine.get_output().unwrap() {
            0 => true,
            1 => false,
            _ => panic!(),
        };

        if should_turn_left {
            facing_direction = match facing_direction {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            }
        } else {
            facing_direction = match facing_direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        };

        match facing_direction {
            Direction::Up => current_coordinate.y -= 1,
            Direction::Right => current_coordinate.x += 1,
            Direction::Down => current_coordinate.y += 1,
            Direction::Left => current_coordinate.x -= 1,
        }

        min_x = min(min_x, current_coordinate.x);
        min_y = min(min_y, current_coordinate.y);
        max_x = max(min_x, current_coordinate.x);
        max_y = max(max_y, current_coordinate.y);
    }

    println!("Painted {} tiles", painted.len());
    for j in min_y..(max_y + 1) {
        for i in min_x..(max_x + 1) {
            let c = match painted.get(&Coordinate { x: i, y: j }) {
                None => ' ',
                Some(c) => match c {
                    Color::White => '█',
                    Color::Black => ' ',
                }
            };

            print!("{}", c);
        }
        println!("");
    }
}

fn main() {
    let program = get_lines("input.txt")[0].clone();
    let registers: Vec<i64> = program
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    // Part 1
    {
        paint_with_starting_tile(Color::Black, &registers);
    }

    // Part 2
    {
        paint_with_starting_tile(Color::White, &registers);
    }
}
