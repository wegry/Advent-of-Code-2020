use std::collections::HashSet;
use std::convert::Infallible;
use std::fs;
use std::str::FromStr;

use regex::Regex;

pub const DAY: u16 = 8;

// acc increases or decreases a single global value called the accumulator by the value given in the argument. For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0. After an acc instruction, the instruction immediately below it is executed next.
// jmp jumps to a new instruction relative to itself. The next instruction to execute is found using the argument as an offset from the jmp instruction; for example, jmp +2 would skip the next instruction, jmp +1 would continue to the instruction immediately below it, and jmp -20 would cause the instruction 20 lines above to be executed next.
// nop stands for No OPeration - it does nothing. The instruction immediately below it is executed next.

#[derive(Clone, Debug, Eq, PartialEq)]
enum OpCode {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for OpCode {
    type Err = Infallible;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\w+) ([-+]\d+)$",).unwrap();
        }
        let captures = RE.captures(raw).unwrap();
        let by = captures[2].parse::<i32>().unwrap();

        let op = match &captures[1] {
            "acc" => OpCode::Acc(by),
            "jmp" => OpCode::Jmp(by),
            "nop" => OpCode::Nop(by),
            x => panic!("[{}] not a valid op code", x),
        };

        Ok(op)
    }
}

#[derive(Clone, Debug)]
struct Program {
    instructions: Vec<OpCode>,
    accumulator: i32,
    current_instruction: usize,
    previously_run_lines: HashSet<usize>,
}

impl Iterator for Program {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let Program {
            current_instruction,
            instructions,
            previously_run_lines,
            ..
        } = self.clone();

        if current_instruction >= instructions.len() {
            return None;
        }

        let op = instructions[current_instruction].clone();

        if previously_run_lines.contains(&current_instruction) {
            // println!(
            //     "Accumulator before second instruction (line {}) run attempt was {}",
            //     current_instruction + 1,
            //     accumulator
            // );
            return None;
        }

        self.previously_run_lines.insert(current_instruction);

        match op {
            OpCode::Acc(acc) => {
                self.accumulator += acc;
                self.current_instruction += 1;
            }
            OpCode::Jmp(by) => {
                if by.is_negative() {
                    self.current_instruction = current_instruction
                        .checked_sub(by.wrapping_abs() as usize)
                        .unwrap();
                } else {
                    self.current_instruction = current_instruction
                        .checked_add(by.wrapping_abs() as usize)
                        .unwrap();
                }
            }
            OpCode::Nop(_) => {
                self.current_instruction += 1;
            }
        };

        Some(self.current_instruction)
    }
}

fn parse_str(raw: &str) -> Program {
    let instructions = raw.lines().map(|l| l.parse::<OpCode>().unwrap()).collect();

    Program {
        accumulator: 0,
        current_instruction: 0,
        instructions,
        previously_run_lines: hashset! {},
    }
}

pub fn part_1() {
    let raw = fs::read_to_string("./src/day8.txt").unwrap();
    let program = parse_str(&raw);

    for acc in program.clone() {
        println!(
            "{:?} | {}",
            program.instructions[program.current_instruction], acc
        )
    }
}

fn bang_on_it_till_it_works(program: Program) -> Option<i32> {
    let original_program = program.clone();
    if program.clone().last() == Some(program.instructions.len()) {
        println!("No changes required");

        return None;
    }

    let indicies_to_toggle = program
        .instructions
        .into_iter()
        .enumerate()
        .filter_map(|(index, instruction)| match instruction {
            OpCode::Acc(_) => None,
            OpCode::Jmp(x) => Some((index, OpCode::Nop(x))),
            OpCode::Nop(x) => Some((index, OpCode::Jmp(x))),
        })
        .collect::<Vec<_>>();

    let target_index = original_program.instructions.len();

    for (index, tweak) in indicies_to_toggle {
        let mut current = original_program.clone();
        current.instructions[index] = tweak.clone();

        if current.clone().last() == Some(target_index) {
            while current.next().is_some() {}
            dbg!("Successful run with", current.accumulator, index, tweak);
            return Some(current.accumulator);
        }
    }

    None
}

pub fn part_2() {
    let raw = fs::read_to_string("./src/day8.txt").unwrap();
    let program = parse_str(&raw);
    bang_on_it_till_it_works(program);
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const RAW_DATA: &str = indoc!(
        "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6"
    );
    #[test]
    fn parse() {
        let parsed = super::parse_str(RAW_DATA);

        for instruction_index in parsed.clone() {
            println!(
                "[{}] {:?} | {}",
                instruction_index,
                parsed.instructions[parsed.current_instruction],
                parsed.accumulator
            )
        }
    }

    #[test]
    fn part_2() {
        let parsed = super::parse_str(RAW_DATA);

        let result = super::bang_on_it_till_it_works(parsed).unwrap();
        assert_eq!(result, 8);
    }
}
