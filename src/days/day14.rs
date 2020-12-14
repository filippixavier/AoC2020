use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

struct BitMask {
    bit: u32,
    sign: bool,
}

enum Instruction {
    Mask(Vec<BitMask>),
    Mem((usize, u64)),
}

impl BitMask {
    fn from_string(input: String) -> Vec<Self> {
        let mut result = vec![];
        for (bit, sign) in input.trim().chars().rev().enumerate() {
            match sign {
                '1' => {
                    result.push(BitMask {
                        bit: bit as u32,
                        sign: true,
                    });
                }
                '0' => {
                    result.push(BitMask {
                        bit: bit as u32,
                        sign: false,
                    });
                }
                'X' => {}
                _ => unreachable!(),
            }
        }

        result
    }
    fn apply(&self, value: u64) -> u64 {
        let bit = (2u64).pow(self.bit);
        let masked_value = value & bit;
        let mut result = value;
        if self.sign && masked_value == 0 {
            result += bit;
        } else if !self.sign && masked_value != 0 {
            result -= bit;
        }
        result
    }
}

fn prepare_input(input: String) -> Vec<Instruction> {
    let mut result = vec![];
    for line in input.trim().lines() {
        let mut parts = line.split(" = ");
        let instruction = parts.next().unwrap();
        let value = parts.next().unwrap();

        if instruction == "mask" {
            result.push(Instruction::Mask(BitMask::from_string(value.to_owned())));
        } else {
            let register = instruction
                .split('[')
                .nth(1)
                .unwrap()
                .split(']')
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            result.push(Instruction::Mem((register, value.parse().unwrap())));
        }
    }
    result
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let instructions = prepare_input(fs::read_to_string(Path::new("./data/day14.txt"))?);
    let mut mask: Vec<BitMask> = vec![];
    let mut registers: HashMap<usize, u64> = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mem((register, value)) => {
                let mut masked_value = value;
                for bit_mask in mask.iter() {
                    masked_value = bit_mask.apply(masked_value);
                }
                registers.insert(register, masked_value);
            }
            Instruction::Mask(new_mask) => {
                mask = new_mask;
            }
        }
    }

    println!("{}", registers.values().sum::<u64>());

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
