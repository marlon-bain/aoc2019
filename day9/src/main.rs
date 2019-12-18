use aoc::utils::get_lines;
use aoc::intcode::IntCodeMachine;

fn main() {
    let program = get_lines("input.txt")[0].clone();
    let registers: Vec<i64> = program.split(',').map(|x| x.parse::<i64>().unwrap()).collect();

    // Part 1
    {
        let mut machine = IntCodeMachine::new(&registers);
        machine.provide_input(1);
        loop {
            match machine.get_output() {
                None => break,
                Some(v) => println!("{}", v)
            };
        }
    }

    // Part 2
    {
        let mut machine = IntCodeMachine::new(&registers);
        machine.provide_input(2);
        loop {
            match machine.get_output() {
                None => break,
                Some(v) => println!("{}", v)
            };
        }
    }
}