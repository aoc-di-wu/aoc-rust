use std::time::Instant;

fn main() {
    let raw_input = include_str!("./input");

    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&raw_input));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&raw_input));
    println!("Finished after {:?}", start.elapsed());
}

fn part1(input: &str) -> usize {
    return input
        .split("\n\n")
        .map(|p| p
            .split_whitespace()
            // Ignore Country ID.
            .filter(|f| !f.starts_with("cid:"))
        )
        .filter(|f| f.clone().count() == 7)
        .count();
}

fn part2(input: &str) -> usize {
    return input
        .split("\n\n")
        .map(|p| p
            .split_whitespace()
            .filter(|f| !f.starts_with("cid:"))
        )
        .filter(|f| f.clone().count() == 7)
        .filter(|f| {
            for field in f.clone() {
                let mut parts = field.split(':');
                let name = parts.next().unwrap();
                let value = parts.next().unwrap();

                match name {
                    "byr" => {
                        let value: usize = value.parse().unwrap();
                        if value < 1920 || 2002 < value {
                            return false;
                        };
                    }
                    "iyr" => {
                        let value: usize = value.parse().unwrap();
                        if value < 2010 || 2020 < value {
                            return false;
                        };
                    }
                    "eyr" => {
                        let value: usize = value.parse().unwrap();
                        if value < 2020 || 2030 < value {
                            return false;
                        };
                    }
                    "hgt" => {
                        if let Some(value) = value.strip_suffix("cm") {
                            let value: usize = value.parse().unwrap();
                            if value < 150 || 193 < value {
                                return false;
                            };
                        } else if let Some(value) = value.strip_suffix("in") {
                            let value: usize = value.parse().unwrap();
                            if value < 59 || 76 < value {
                                return false;
                            };
                        } else {
                            return false;
                        }
                    }
                    "hcl" => {
                        if value.len() != 7 ||
                            !value.starts_with('#') ||
                            !value[1..]
                                .chars()
                                .all(|c| c.is_digit(16)) {
                            return false;
                        }
                    }
                    "ecl" => {
                        if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value) {
                            return false;
                        }
                    }
                    "pid" => {
                        if value.len() != 9 ||
                            !value
                                .chars()
                                .all(|c| c.is_digit(10)) {
                            return false;
                        }
                    }
                    _ => unreachable!(),
                };
            }
            return true;
        })
        .count();
}

#[cfg(test)]
mod test {
    use crate::part1;
    use crate::part2;

    #[test]
    fn p1e1() {
        assert_eq!(part1("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"), 2);
    }

    #[test]
    fn p2e1() {
        assert_eq!(part2("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"), 0);
    }

    #[test]
    fn p2e2() {
        assert_eq!(part2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"), 4);
    }
}


