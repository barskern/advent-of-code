use aoc_runner_derive::*;

use std::collections::HashMap as Map;

type Passport = Map<String, String>;

#[aoc_generator(day4)]
fn gen(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|s| {
            s.split_whitespace()
                .filter_map(|f| {
                    let mut it = f.split(':');
                    Some((it.next()?.to_owned(), it.next()?.to_owned()))
                })
                .collect::<Passport>()
        })
        .collect()
}

#[rustfmt::skip]
const REQUIRED_FIELDS: &[(&str, fn(&str) -> bool)] = &[
    //("cid", |_| true), // (Country ID) **IGNORED**
    ("byr", |s| s.parse().map(|byr| 1920 <= byr && byr <= 2002).unwrap_or(false)), // (Birth Year)
    ("iyr", |s| s.parse().map(|iyr| 2010 <= iyr && iyr <= 2020).unwrap_or(false)), // (Issue Year)
    ("eyr", |s| s.parse().map(|eyr| 2020 <= eyr && eyr <= 2030).unwrap_or(false)), // (Expiration Year)
    ("hgt", |s| {
        if s.ends_with("cm") {
            s.trim_end_matches("cm").parse().map(|cm| 150 <= cm && cm <= 193).unwrap_or(false)
        } else if s.ends_with("in") {
            s.trim_end_matches("in").parse().map(|in_| 59 <= in_ && in_ <= 76).unwrap_or(false)
        } else {
            false
        }
    }), // (Height)
    ("hcl", |s| s.len() == 7 && s.starts_with("#") && s[1..].chars().all(|c| c.is_digit(16))), // (Hair Color)
    ("ecl", |s| matches!(s, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")), // (Eye Color)
    ("pid", |s| s.len() == 9 && s.chars().all(|c| c.is_digit(10))), // (Passport ID)
];

#[aoc(day4, part1)]
pub fn part1(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|passport| {
            REQUIRED_FIELDS
                .iter()
                .all(|&(field_name, _)| passport.contains_key(field_name))
        })
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Map<String, String>]) -> usize {
    input
        .iter()
        .filter(|passport| {
            REQUIRED_FIELDS.iter().all(|&(field_name, validate)| {
                passport
                    .get(field_name)
                    .map(|field| validate(field))
                    .unwrap_or(false)
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_PASSWORDS: &str = r#"
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;

    #[test]
    fn part2_test() {
        assert_eq!(4, part2(&gen(VALID_PASSWORDS)));
    }
}
