use aoc::intcode::IntCodeMachine;
use aoc::intcode::IntCodeError;
use aoc::utils::get_lines;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

struct Velocity {
    x: i32,
    y: i32,
}

enum BlockType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl BlockType {
    fn from(v: i64) -> Self {
        match v {
            0 => BlockType::Empty,
            1 => BlockType::Wall,
            2 => BlockType::Block,
            3 => BlockType::Paddle,
            4 => BlockType::Ball,
            _ => panic!(),
        }
    }

    fn toChar(&self) -> char {
        match self {
            BlockType::Empty => ' ',
            BlockType::Wall => 'W',
            BlockType::Block => '.',
            BlockType::Paddle => '_',
            BlockType::Ball => '$',
        }
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
        let mut block_count = 0;
        let mut computer = IntCodeMachine::new(&registers);
        let mut screen: HashMap<Position, BlockType> = HashMap::new();

        loop {
            let x = match computer.get_output() {
                None => break,
                Some(v) => v,
            };

            let y = computer.get_output().unwrap();
            let block_type = BlockType::from(computer.get_output().unwrap());

            screen.insert(Position { x, y }, block_type);
        }

        for (_, block_type) in screen.iter() {
            match block_type {
                BlockType::Block => block_count += 1,
                _ => (),
            }
        }

        println!("{}", block_count);
    }

    // Part 2
    {
        let mut hacked_registers = registers.clone();
        hacked_registers[0] = 2;

        let mut computer = IntCodeMachine::new(&hacked_registers);
        let mut score = 0;

        let mut ball_x = 0;
        let mut paddle_x = 0;

        let mut gap = 0;

        'outer: loop {
            let mut x = 1;
            loop {
                match computer.get_output_v2() {
                    Err(IntCodeError::ProgramComplete) => break 'outer,
                    Err(IntCodeError::NeedInput) => {
                        computer.provide_input(match paddle_x == ball_x {
                            true => 0,
                            false => match paddle_x < ball_x {
                                true => 1,
                                false => -1
                            }
                        });
                    },
                    Ok(v) => {
                        x = v;
                        break;
                    }
                }
            }

            computer.get_output();
            let value = computer.get_output().unwrap();

            if (x == -1) {
                score = value;
            } else {
                match BlockType::from(value) {
                    BlockType::Ball => {
                        ball_x = x;
                    },
                    BlockType::Paddle => paddle_x = x,
                    _ => ()
                }
            }
        }

        println!("{}", score);
    }
}
