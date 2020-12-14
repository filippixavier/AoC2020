use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

struct BitMask {
    bit: u32,
    sign: Option<bool>,
}

enum Instruction {
    Mask(Vec<BitMask>),
    Mem((u64, u64)),
}

impl BitMask {
    fn from_string(input: String) -> Vec<Self> {
        let mut result = vec![];
        for (bit, sign) in input.trim().chars().enumerate() {
            match sign {
                '1' => {
                    result.push(BitMask {
                        bit: bit as u32,
                        sign: Some(true),
                    });
                }
                '0' => {
                    result.push(BitMask {
                        bit: bit as u32,
                        sign: Some(false),
                    });
                }
                'X' => {
                    result.push(BitMask {
                        bit: bit as u32,
                        sign: None,
                    });
                }
                _ => unreachable!(),
            }
        }

        result
    }
    fn apply(&self, value: u64) -> u64 {
        let bit = (2u64).pow(self.bit);
        let masked_value = value & bit;
        let mut result = value;

        let sign = if let Some(sign) = self.sign {
            sign
        } else {
            return value;
        };

        if sign && masked_value == 0 {
            result += bit;
        } else if !sign && masked_value != 0 {
            result -= bit;
        }
        result
    }

    fn apply_register(&self, value: String) -> String {
        let sign = if let Some(sign) = self.sign {
            sign
        } else {
            return value
                .chars()
                .enumerate()
                .map(|(index, value)| {
                    if index == self.bit as usize {
                        'X'
                    } else {
                        value
                    }
                })
                .collect();
        };

        if sign {
            return value
                .chars()
                .enumerate()
                .map(|(index, value)| {
                    if index == self.bit as usize {
                        '1'
                    } else {
                        value
                    }
                })
                .collect();
        }

        value
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
                .parse()
                .unwrap();
            result.push(Instruction::Mem((register, value.parse().unwrap())));
        }
    }
    result
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let instructions = prepare_input(fs::read_to_string(Path::new("./data/day14.txt"))?);
    let mut mask: Vec<BitMask> = vec![];
    let mut registers: HashMap<u64, u64> = HashMap::new();

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

    println!(
        "Sum of value should be: {}",
        registers.values().sum::<u64>()
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let instructions = prepare_input(fs::read_to_string(Path::new("./data/day14.txt"))?);
    let mut mask: Vec<BitMask> = vec![];
    let mut registers: HashMap<u64, u64> = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mem((register, value)) => {
                let mut registers_bits = format!("{:036b}", register);
                let mut positions: Vec<usize> = vec![];
                for bit_mask in mask.iter() {
                    registers_bits = bit_mask.apply_register(registers_bits);
                }
                let len = registers_bits.len() - 1;
                let base_register = registers_bits
                    .chars()
                    .enumerate()
                    .map(|(index, value)| {
                        if value == 'X' {
                            positions.push(len - index);
                            return '0';
                        }
                        value
                    })
                    .collect::<String>();
                let base_register = u64::from_str_radix(base_register.as_str(), 2).unwrap();

                let num_of_permutations = (2usize).pow(positions.len() as u32);

                for i in 0..num_of_permutations {
                    let binary = format!("{:b}", i);
                    let ones = binary
                        .chars()
                        .rev()
                        .enumerate()
                        .filter(|(_, value)| *value == '1')
                        .map(|(index, _)| index)
                        .collect::<Vec<_>>();
                    let register = ones.iter().fold(0, |acc, elem| {
                        let val = positions[*elem];
                        acc + (2u64).pow(val as u32)
                    });
                    registers.insert(base_register + register, value);
                }
            }
            Instruction::Mask(new_mask) => {
                mask = new_mask;
            }
        }
    }

    println!(
        "Using the v2 algorithm, sum should be: {}",
        registers.values().sum::<u64>()
    );
    Ok(())
}
