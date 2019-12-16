use aoc::utils::get_lines;

enum OpCode {
    ADD = 1,
    MULTIPLY = 2,
    END = 99
}

fn from_i32(i: i32) -> OpCode {
    match i {
        1 => OpCode::ADD,
        2 => OpCode::MULTIPLY,
        99 => OpCode::END,
        _ => panic!("wtf")
    }
}

fn run_intcodes(external_registers: &Vec<i32>, a: i32, b: i32) -> i32 {
    let mut registers: Vec<i32> = external_registers.clone();
    registers[1] = a;
    registers[2] = b;

    let mut index = 0;
    loop {
        match from_i32(registers[index]) {
            OpCode::END => break,
            OpCode::ADD => {
                let target = registers[index + 3] as usize;
                registers[target] = registers[registers[index + 1] as usize] + registers[registers[index + 2] as usize];
            },
            OpCode::MULTIPLY => {
                let target = registers[index + 3] as usize;
                registers[target] = registers[registers[index + 1] as usize] * registers[registers[index + 2] as usize];
            }
        }

        index += 4;
    }

    registers[0]
}

fn main() {
    let program = get_lines("input.txt")[0].clone();
    let registers: Vec<i32> = program.split(',').map(|x| x.parse().unwrap()).collect();

    // Part 1
    {
        println!("{}", run_intcodes(&registers, 12, 2));
    }

    // Part 2
    {
        for verb in 1..100 {
            for noun in 1..100 {
                if run_intcodes(&registers, noun, verb) == 19690720 {
                    println!("{}", 100 * noun + verb);
                }
            }
        }
    }
}
