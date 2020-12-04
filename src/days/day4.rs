use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn from_string(input: String) -> Passport {
        let mut passport = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };
        for passport_field in input.split_whitespace() {
            let field = passport_field.split(':').collect::<Vec<_>>();
            match field[0] {
                "byr" => {
                    passport.byr = Some(field[1].parse().unwrap());
                }
                "iyr" => {
                    passport.iyr = Some(field[1].parse().unwrap());
                }
                "eyr" => {
                    passport.eyr = Some(field[1].parse().unwrap());
                }
                "hgt" => {
                    passport.hgt = Some(String::from(field[1]));
                }
                "hcl" => {
                    passport.hcl = Some(String::from(field[1]));
                }
                "ecl" => {
                    passport.ecl = Some(String::from(field[1]));
                }
                "pid" => {
                    passport.pid = Some(String::from(field[1]));
                }
                "cid" => {
                    passport.cid = Some(String::from(field[1]));
                }
                _ => unreachable!(),
            }
        }

        passport
    }

    fn is_valid(&self) -> bool {
        if !(self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some())
        {
            return false;
        }
        if let Some(byr) = self.byr {
            if byr < 1920 || byr > 2002 {
                return false;
            }
        }

        if let Some(iyr) = self.iyr {
            if iyr < 2010 || iyr > 2020 {
                return false;
            }
        }

        if let Some(eyr) = self.eyr {
            if eyr < 2020 || eyr > 2030 {
                return false;
            }
        }

        if let Some(hgt) = &self.hgt {
            let reg = Regex::new(r"(\d+)(in|cm)").unwrap();
            if !reg.is_match(&hgt) {
                return false;
            }
            if let Some(capture) = reg.captures(&hgt) {
                let value = capture[1].parse::<usize>().unwrap();
                match &capture[2] {
                    "in" => {
                        if value < 59 || value > 76 {
                            return false;
                        }
                    }
                    "cm" => {
                        if value < 150 || value > 193 {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
        }

        if let Some(hcl) = &self.hcl {
            if hcl.chars().count() != 7 {
                return false;
            }

            let mut hex_value = hcl.chars();

            if hex_value.next().unwrap() != '#' {
                return false;
            }

            let value =
                i32::from_str_radix(hex_value.collect::<String>().as_str(), 16).unwrap_or(-1);
            let max_value = i32::from_str_radix("ffffff", 16).unwrap();

            if value < 0 || value > max_value {
                return false;
            }
        }

        match self.ecl.as_ref().unwrap().as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
            _ => return false,
        }

        if let Some(pid) = &self.pid {
            if pid.chars().count() != 9 || pid.parse::<i32>().unwrap_or(-1) < 0 {
                return false;
            }
        }

        true
    }
}

fn prepare_input(input: String) -> Vec<Passport> {
    let mut passports = vec![];
    for passport_block in input.split("\r\n\r\n") {
        let passport = Passport::from_string(String::from(passport_block));
        passports.push(passport);
    }

    passports
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let input = prepare_input(fs::read_to_string(Path::new("./data/day4.txt"))?);
    let valid_passports = input
        .iter()
        .filter(|passport| {
            passport.byr.is_some()
                && passport.iyr.is_some()
                && passport.eyr.is_some()
                && passport.hgt.is_some()
                && passport.hcl.is_some()
                && passport.ecl.is_some()
                && passport.pid.is_some()
        })
        .count();

    println!("Number of valid passports: {}", valid_passports);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let input = prepare_input(fs::read_to_string(Path::new("./data/day4.txt"))?);
    let valid_passports = input.iter().filter(|passport| passport.is_valid());

    println!(
        "Number of *really* valid passports: {}",
        valid_passports.count()
    );
    Ok(())
}
