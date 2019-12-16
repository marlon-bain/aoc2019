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
    RelativeBaseOffset,
    End,
}

impl From<i64> for OpCode {
    fn from(i: i64) -> Self {
        match i {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            9 => OpCode::RelativeBaseOffset,
            99 => OpCode::End,
            _ => panic!("wtf opcode {}", i),
        }
    }
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for ParameterMode {
    fn from(i: i64) -> Self {
        match i {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("wtf parameter mode {}", i),
        }
    }
}

fn get_parameter(
    registers: &Vec<i64>,
    index: i64,
    number: i64,
    relative_base: i64,
    mode: ParameterMode,
) -> i64 {
    match mode {
        ParameterMode::Position => registers[registers[index as usize + number as usize] as usize],
        ParameterMode::Immediate => registers[index as usize + number as usize],
        ParameterMode::Relative => {
            registers[(registers[(index + number) as usize] + relative_base) as usize]
        }
    }
}

fn get_parameter_as_address(
    registers: &Vec<i64>,
    index: i64,
    number: i64,
    relative_base: i64,
    mode: ParameterMode,
) -> usize {
    let result = match mode {
        ParameterMode::Position => registers[index as usize + number as usize],
        ParameterMode::Relative => {
            registers[(index + number) as usize] + relative_base
        },
        ParameterMode::Immediate => panic!("Not possible"),
    };

    result as usize
}

fn run_program(
    external_registers: &Vec<String>,
    external_input: &Vec<i64>,
    write_out: Option<&mut Vec<String>>,
    starting_index: &mut i64,
    external_relative_base: &mut i64,
) -> Option<i64> {
    let input_vec = external_input.clone();
    let mut input = input_vec.iter();
    let mut output: Vec<i64> = external_registers
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    output.extend(0..10000);

    let mut index: i64 = *starting_index;
    let mut result = None;
    let mut relative_base = *external_relative_base;

    loop {
        let instruction: Vec<String> = output[index as usize]
            .to_string()
            .chars()
            .rev()
            .map(|x| x.to_string())
            .collect();

        let parameterModeA = ParameterMode::from(
            instruction
                .get(2)
                .unwrap_or(&String::from("0"))
                .parse::<i64>()
                .unwrap(),
        );
        let parameterModeB = ParameterMode::from(
            instruction
                .get(3)
                .unwrap_or(&String::from("0"))
                .parse::<i64>()
                .unwrap(),
        );
        let parameterModeC = ParameterMode::from(
            instruction
                .get(4)
                .unwrap_or(&String::from("0"))
                .parse::<i64>()
                .unwrap(),
        );

        let mut step: i64;
        let opcodeDigit = instruction.get(0).unwrap().parse::<i64>().unwrap()
            + instruction
                .get(1)
                .unwrap_or(&String::from("0"))
                .parse::<i64>()
                .unwrap()
                * 10;
        match OpCode::from(opcodeDigit) {
            OpCode::End => break,
            OpCode::Add => {
                step = 4;
                let leftOperand = get_parameter(&output, index, 1, relative_base, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, relative_base, parameterModeB);
                let target =
                    get_parameter_as_address(&output, index, 3, relative_base, parameterModeC);

                output[target as usize] = leftOperand + rightOperand;
            }
            OpCode::Multiply => {
                step = 4;
                let leftOperand = get_parameter(&output, index, 1, relative_base, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, relative_base, parameterModeB);
                let target =
                    get_parameter_as_address(&output, index, 3, relative_base, parameterModeC);

                output[target as usize] = leftOperand * rightOperand;
            }
            OpCode::Input => {
                step = 2;
                let target =
                    get_parameter_as_address(&output, index, 1, relative_base, parameterModeA);

                output[target as usize] = *input.next().unwrap();
            }
            OpCode::Output => {
                step = 2;
                let operand =
                    get_parameter(&output, index, 1, relative_base, parameterModeA);

                result = Some(operand);
                index += step;
                break;
            }
            OpCode::JumpIfTrue => {
                step = 3;
                let leftOperand = get_parameter(&output, index, 1, relative_base, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, relative_base, parameterModeB);

                if leftOperand != 0 {
                    index = rightOperand;
                    continue;
                }
            }
            OpCode::JumpIfFalse => {
                step = 3;
                let leftOperand = get_parameter(&output, index, 1, relative_base, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, relative_base, parameterModeB);

                if leftOperand == 0 {
                    index = rightOperand;
                    continue;
                }
            }
            OpCode::LessThan => {
                step = 4;
                let leftOperand = get_parameter(&output, index, 1, relative_base, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, relative_base, parameterModeB);
                let target =
                    get_parameter_as_address(&output, index, 3, relative_base, parameterModeC);

                if leftOperand < rightOperand {
                    output[target as usize] = 1;
                } else {
                    output[target as usize] = 0;
                }
            }
            OpCode::Equals => {
                step = 4;
                let leftOperand = get_parameter(&output, index, 1, relative_base, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, relative_base, parameterModeB);
                let target =
                    get_parameter_as_address(&output, index, 3, relative_base, parameterModeC);

                if leftOperand == rightOperand {
                    output[target as usize] = 1;
                } else {
                    output[target as usize] = 0;
                }
            }
            OpCode::RelativeBaseOffset => {
                step = 2;
                let operand = get_parameter(&output, index, 1, relative_base, parameterModeA);
                relative_base += operand;
            }
        }

        index += step;
    }

    if let Some(r) = write_out {
        for i in 0..r.len() {
            r[i] = output[i].to_string();
        }
    }

    *starting_index = index;
    *external_relative_base = relative_base;
    result
}

fn main() {
    let program = get_lines("input.txt")[0].clone();
    let mut registers: Vec<String> = program.split(',').map(|x| x.to_owned()).collect();

    // Part 1
    {
        let input: Vec<i64> = vec![1];
        let mut index = 0;
        let mut relative_base = 0;
        println!("{}", run_program(&registers.clone(), &input, Some(&mut registers), &mut 0, &mut relative_base).unwrap());
    }

    // Part 2
    {
        let input: Vec<i64> = vec![2];
        let mut index = 0;
        let mut relative_base = 0;
        println!("{}", run_program(&registers.clone(), &input, Some(&mut registers), &mut 0, &mut relative_base).unwrap());
    }
}