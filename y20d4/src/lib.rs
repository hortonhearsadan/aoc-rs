extern crate helper;
use helper::{print_part_1, print_part_2, FromInput};
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
                _ => (),
            }
        }

        Ok(Passport {
            raw_byr,
            raw_iyr,
            raw_eyr,
            raw_hgt,
            raw_hcl,
            raw_ecl,
            raw_pid,
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
        if let Ok(number) = num.parse::<u32>() {
            return VALID_HGT_UNIT.contains(&unit)
                && match *unit {
                    "cm" => (150..=193).contains(&number),
                    "in" => (59..=76).contains(&number),
                    _ => false,
                };
        }
        false
    }

    fn hcl_is_valid(&self) -> bool {
        let hcl = self.raw_hcl.as_ref().unwrap();
        let (hash, hexcode) = hcl.split_at(1);

        hash == "#" && hexcode.len() == 6 && hexcode.chars().all(|c| c.is_alphanumeric())
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
    if let Ok(yr) = year {
        year_string.len() == 4 && (min..=max).contains(&yr)
    } else {
        false
    }
}

const FILENAME: &str = env!("CARGO_PKG_NAME");
pub fn main() {
    let passports = Passport::from_multiline_input(FILENAME);

    let count = passports.iter().filter(|p| p.is_valid()).count();
    print_part_1(count);

    let count = passports.iter().filter(|p| p.is_really_valid()).count();
    print_part_2(count)
}

#[cfg(test)]
mod tests {
    use crate::{valid_year, Passport};
    use std::str::FromStr;

    #[test]
    fn test_valid_year() {
        let year_string = "2020";
        let min = 2000;
        let max = 2025;
        assert_eq!(true, valid_year(year_string, min, max))
    }
    #[test]
    fn test_invalid_year() {
        let year_string = "2031";
        let min = 2000;
        let max = 2025;
        assert_eq!(false, valid_year(year_string, min, max))
    }

    #[test]
    fn from_str() {
        let str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        let p = Passport::from_str(str).unwrap();
        assert_eq!(p.raw_ecl, Some("gry".to_owned()));
        assert_eq!(p.raw_pid, Some("860033327".to_owned()));
        assert_eq!(p.raw_eyr, Some("2020".to_owned()));
        assert_eq!(p.raw_hcl, Some("#fffffd".to_owned()));
        assert_eq!(p.raw_byr, Some("1937".to_owned()));
        assert_eq!(p.raw_iyr, Some("2017".to_owned()));
        assert_eq!(p.raw_hgt, Some("183cm".to_owned()));
    }

    #[test]
    fn valid_hgt_cm_is_valid() {
        let p = Passport {
            raw_hgt: Some("180cm".to_owned()),
            ..Default::default()
        };
        assert_eq!(true, p.hgt_is_valid())
    }

    #[test]
    fn valid_hgt_in_is_valid() {
        let p = Passport {
            raw_hgt: Some("70in".to_owned()),
            ..Default::default()
        };
        assert_eq!(true, p.hgt_is_valid())
    }

    #[test]
    fn invalid_hgt_is_valid() {
        let p = Passport {
            raw_hgt: Some("abin".to_owned()),
            ..Default::default()
        };
        assert_eq!(false, p.hgt_is_valid())
    }
    #[test]
    fn invalid_too_tall_hgt_is_valid() {
        let p = Passport {
            raw_hgt: Some("100in".to_owned()),
            ..Default::default()
        };
        assert_eq!(false, p.hgt_is_valid())
    }

    #[test]
    fn hcl_is_valid() {
        let p = Passport {
            raw_hcl: Some("#abc123".to_owned()),
            ..Default::default()
        };
        assert_eq!(true, p.hcl_is_valid())
    }
}
