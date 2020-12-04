use std::error::Error;
use std::fs;
use std::path::Path;

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
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
                    passport.byr = Some(String::from(field[1]));
                }
                "iyr" => {
                    passport.iyr = Some(String::from(field[1]));
                }
                "eyr" => {
                    passport.eyr = Some(String::from(field[1]));
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
    let valid_passwords = input
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

    println!("Number of valid passwords: {}", valid_passwords);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    // let input = fs::read_to_string(Path::new("./data/day4.txt"))?;
    Ok(())
}
