use aoc::utils::get_lines;

enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    End
}

impl From<i32> for OpCode {
    fn from(i: i32) -> Self {
        match i {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            99 => OpCode::End,
            _ => panic!("wtf opcode {}", i)
        }
    }
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate
}

impl From<i32> for ParameterMode {
    fn from(i: i32) -> Self {
        match i {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("wtf parameter mode {}", i )
        }
    }
}

fn get_parameter(registers: &Vec<i32>, index: i32, number: i32, mode: ParameterMode) -> i32 {
    match mode {
        ParameterMode::Position => {
            registers[registers[index as usize + number as usize] as usize]
        },
        ParameterMode::Immediate => registers[index as usize + number as usize]
    }
}

fn run_program(external_registers: &Vec<String>, id: i32) {
    let mut output: Vec<i32> = external_registers.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut index: i32 = 0;
    loop {
        let instruction: Vec<String> = output[index as usize].to_string().chars().rev().map(|x| x.to_string()).collect();

        let parameterModeA = ParameterMode::from(instruction.get(2).unwrap_or(&String::from("0")).parse::<i32>().unwrap());
        let parameterModeB = ParameterMode::from(instruction.get(3).unwrap_or(&String::from("0")).parse::<i32>().unwrap());
        let parameterModeC = ParameterMode::from(instruction.get(4).unwrap_or(&String::from("0")).parse::<i32>().unwrap());

        let mut step: i32;
        let opcodeDigit = instruction.get(0).unwrap().parse::<i32>().unwrap() + instruction.get(1).unwrap_or(&String::from("0")).parse::<i32>().unwrap() * 10;
        match OpCode::from(opcodeDigit) {
            OpCode::End => break,
            OpCode::Add => {
                step = 4;
                let leftOperand = get_parameter(&output, index, 1, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, parameterModeB);
                let target = get_parameter(&output, index, 3, ParameterMode::Immediate);

                output[target as usize] = leftOperand + rightOperand;
            },
            OpCode::Multiply => {
                step = 4;
                let leftOperand = get_parameter(&output, index, 1, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, parameterModeB);
                let target = get_parameter(&output, index, 3, ParameterMode::Immediate);

                output[target as usize] = leftOperand * rightOperand;
            },
            OpCode::Input => {
                step = 2;
                let target = get_parameter(&output, index, 1, ParameterMode::Immediate);

                // HARDCODED because I ain't about figuring out rust io
                output[target as usize] = id;
            },
            OpCode::Output => {
                step = 2;
                let target = get_parameter(&output, index, 1, ParameterMode::Immediate);

                println!("{}", output[target as usize]);
            },
            OpCode::JumpIfTrue => {
                step = 3;
                let leftOperand = get_parameter(&output, index, 1, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, parameterModeB);

                if leftOperand != 0 {
                    index = rightOperand;
                    continue;
                }
            },
            OpCode::JumpIfFalse => {
                step = 3;
                let leftOperand = get_parameter(&output, index, 1, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, parameterModeB);

                if leftOperand == 0 {
                    index = rightOperand;
                    continue;
                }
            },
            OpCode::LessThan => {
                step = 4;
                let leftOperand = get_parameter(&output, index, 1, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, parameterModeB);
                let target = get_parameter(&output, index, 3, ParameterMode::Immediate);

                if (leftOperand < rightOperand) {
                    output[target as usize] = 1;
                } else {
                    output[target as usize] = 0;
                }
            },
            OpCode::Equals => {
                step = 4;
                let leftOperand = get_parameter(&output, index, 1, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, parameterModeB);
                let target = get_parameter(&output, index, 3, ParameterMode::Immediate);

                if (leftOperand == rightOperand) {
                    output[target as usize] = 1;
                } else {
                    output[target as usize] = 0;
                }
            }
        }

        index += step;
    }
}

fn main() {
    let program = get_lines("input.txt")[0].clone();
    let registers: Vec<String> = program.split(',').map(|x| x.to_owned()).collect();

    // Part 1
    {
        run_program(&registers, 1);
    }

    // Part 2
    {
        run_program(&registers, 5);
    }
}
