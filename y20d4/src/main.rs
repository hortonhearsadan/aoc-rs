extern crate helper;
use helper::{get_raw_input, print_part_1, print_part_2};
use std::str::FromStr;

const VALID_ECL: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
const VALID_HGT_UNIT: [&str; 2] = ["cm", "in"];

#[derive(Default)]
struct Passport {
    raw_byr: Option<String>,
    raw_iyr: Option<String>,
    raw_eyr: Option<String>,
    raw_hgt: Option<String>,
    raw_hcl: Option<String>,
    raw_ecl: Option<String>,
    raw_pid: Option<String>,
    raw_cid: Option<String>,
}

impl FromStr for Passport {
    type Err = std::string::ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut raw_byr: Option<String> = None;
        let mut raw_iyr: Option<String> = None;
        let mut raw_eyr: Option<String> = None;
        let mut raw_hgt: Option<String> = None;
        let mut raw_hcl: Option<String> = None;
        let mut raw_ecl: Option<String> = None;
        let mut raw_pid: Option<String> = None;
        let mut raw_cid: Option<String> = None;

        let new_str = string.replace('\n', " ");
        let new_vec = new_str.split_whitespace();
        for str in new_vec {
            let key_value: Vec<_> = str.split(':').collect();
            let key = key_value[0];
            let value = key_value[1].to_string();
            match key {
                "byr" => raw_byr = Some(value),
                "iyr" => raw_iyr = Some(value),
                "eyr" => raw_eyr = Some(value),
                "hgt" => raw_hgt = Some(value),
                "hcl" => raw_hcl = Some(value),
                "ecl" => raw_ecl = Some(value),
                "pid" => raw_pid = Some(value),
                "cid" => raw_cid = Some(value),
                _ => (),
            }
        }

        Ok(Passport {
            raw_cid,
            raw_pid,
            raw_ecl,
            raw_hcl,
            raw_hgt,
            raw_eyr,
            raw_iyr,
            raw_byr,
        })
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.raw_byr.is_some()
            && self.raw_iyr.is_some()
            && self.raw_eyr.is_some()
            && self.raw_hgt.is_some()
            && self.raw_hcl.is_some()
            && self.raw_ecl.is_some()
            && self.raw_pid.is_some()
    }

    fn is_really_valid(&self) -> bool {
        self.is_valid()
            && self.byr_is_valid()
            && self.iyr_is_valid()
            && self.eyr_is_valid()
            && self.hgt_is_valid()
            && self.hcl_is_valid()
            && self.ecl_is_valid()
            && self.pid_is_valid()
    }

    fn byr_is_valid(&self) -> bool {
        valid_year(&self.raw_byr.as_ref().unwrap(), 1920, 2002)
    }

    fn iyr_is_valid(&self) -> bool {
        valid_year(&self.raw_iyr.as_ref().unwrap(), 2010, 2020)
    }

    fn eyr_is_valid(&self) -> bool {
        valid_year(&self.raw_eyr.as_ref().unwrap(), 2020, 2030)
    }

    fn hgt_is_valid(&self) -> bool {
        let len = self.raw_hgt.as_ref().unwrap().len();
        let (num, unit) = &self.raw_hgt.as_ref().unwrap().split_at(len - 2);
        num.parse::<f32>().is_ok() && VALID_HGT_UNIT.contains(&unit)
    }

    fn hcl_is_valid(&self) -> bool {
        let hcl = self.raw_hcl.as_ref().unwrap();
        &hcl[0..1] == "#" && hcl.len() == 6 && hcl.chars().all(|c| c.is_alphanumeric())
    }

    fn ecl_is_valid(&self) -> bool {
        let ecl = self.raw_ecl.as_ref().unwrap();
        VALID_ECL.contains(&&**ecl)
    }

    fn pid_is_valid(&self) -> bool {
        let pid = &self.raw_pid.as_ref().unwrap();
        pid.chars().all(|c| c.is_numeric()) && pid.len() == 9
    }
}

fn valid_year(year_string: &str, min: i32, max: i32) -> bool {
    let year = year_string.parse::<i32>();
    return if let Ok(yr) = year {
        year_string.len() == 4 && yr <= max && yr >= min
    } else {
        false
    };
}

const FILENAME: &str = env!("CARGO_PKG_NAME");
fn main() {
    let input = get_raw_input(FILENAME);
    let blobs = input.split("\n\n").collect::<Vec<_>>();
    let passports: Vec<Passport> = blobs
        .iter()
        .map(|b| Passport::from_str(*b).unwrap())
        .collect();

    let count = passports.iter().filter(|p| p.is_valid()).count();
    print_part_1(count);

    let count = passports.iter().filter(|p| p.is_really_valid()).count();
    print_part_2(count)
}
