
pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let parsed = prepare_data(&data);

        let count = parsed.iter()
            .map(|x| validate(x))
            .reduce(|a, b| a+b );

        tracing::info!("{}", count.unwrap());
    }
    
    fn p2(&self, data: String) {
        let parsed = p2_prepare_data(&data);
        let count = parsed.iter()
            .map(|x| p2_validate(x))
            .reduce(|a, b| a+b );

        tracing::info!("{}", count.unwrap());
    }
}

fn prepare_data(raw: &str) -> Vec<Vec<&str>> {
    raw.split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| {
            x
                .split_whitespace()
                .map(|y| {
                    y.split(":").collect::<Vec<&str>>()[0]
                }) 
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>()
}


fn validate(passport: &Vec<&str>) -> u8 {
    let keys = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
        // "cid",
    ];

    for key in keys {
        if !passport.contains(&key) {
            return 0
        }
    }

    return 1
}


fn p2_prepare_data(raw: &str) -> Vec<Vec<Vec<&str>>> {
    raw
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| {
            x
                .split_whitespace()
                .map(|y| {
                    y.split(":").collect::<Vec<&str>>()
                }) 
                .collect::<Vec<Vec<&str>>>()
        })
        .collect::<Vec<Vec<Vec<&str>>>>()
}



fn p2_strict_val(val: &Vec<&str>) -> u32 {
    
    match val[0] {
        "byr" => { 
            // byr (Birth Year) - four digits; at least 1920 and at most 2002.
            if val[1].chars().count() == 4 && val[1] >= "1920" && val[1] <= "2002"{
                return 1;
            }
        },
        "iyr" => { 
            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            if val[1].chars().count() == 4 && val[1] >= "2010" && val[1] <= "2020"{
                return 1
            } 
        },
        "eyr" => { 
            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            if val[1].chars().count() == 4 && val[1] >= "2020" && val[1] <= "2030"{
                return 1;
            }
        },
        "hgt" => { 
            // hgt (Height) - a number followed by either cm or in:
            if val[1].contains("cm") && val[1].chars().count() == 5 {
                // If cm, the number must be at least 150 and at most 193.
                let num = &val[1][0..3];
                if num >= "150" || num <= "193" {
                    return 1;
                } 
            } else if val[1].contains("in") && val[1].chars().count() == 4 {
                // If in, the number must be at least 59 and at most 76.
                let num = &val[1][0..2];
                if num >= "59" || num <= "76" {
                    return 1;
                } 
            } 
        },
        "hcl" => { 
            // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            let hex = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
            if val[1].starts_with("#") && val[1].chars().count() == 7 {
                for i in val[1][1..].chars().into_iter() {
                    if !hex.contains(&i) {
                        return 0;
                    }
                }

                return 1;
            }
        },
        "ecl" => { 
            // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            if colors.contains(&val[1]) {
                return 1
            }
        },
        "pid" => { 
            // pid (Passport ID) - a nine-digit number, including leading zeroes.
            if val[1].chars().count() == 9 && val[1] >= "000000000" && val[1] <= "999999999" {
                return 1
            }
        },
        _ => { return 0 }
    }

    return 0
}



fn p2_validate(passport: &Vec<Vec<&str>>) -> u32 {
    let mut res = 0;

    for val in passport {
        res += p2_strict_val(val);
    }
    
    if res == 7 {1} else {0}
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_rust::Day;

    #[test]
    fn data_parser() {

        let raw = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let res = vec![
            vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "cid", "hgt"], 
            vec!["iyr", "ecl", "cid", "eyr", "pid", "hcl", "byr"], 
            vec!["hcl", "iyr", "eyr", "ecl", "pid", "byr", "hgt"], 
            vec!["hcl", "eyr", "pid", "iyr", "ecl", "hgt"],
        ];

        assert_eq!(res, prepare_data(raw));
    }

    #[test]
    fn validations() {
        assert_eq!(1, validate(&vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "cid", "hgt"]));
        assert_eq!(0, validate(&vec!["iyr", "ecl", "cid", "eyr", "pid", "hcl", "byr"]));
        assert_eq!(1, validate(&vec!["hcl", "iyr", "eyr", "ecl", "pid", "byr", "hgt"]));
        assert_eq!(0, validate(&vec!["hcl", "eyr", "pid", "iyr", "ecl", "hgt"]));
    }

    #[test]
    fn valid_counter() {
        let _raw = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        // assert_eq!(2, AoC.p1(raw));
    }

    #[test]
    fn p2_valid_data() {
        assert_eq!(1, p2_strict_val(&vec!["byr",   "2002"]));
        assert_eq!(0, p2_strict_val(&vec!["byr", "2003"]));
        assert_eq!(1, p2_strict_val(&vec!["hgt",   "60in"]));
        assert_eq!(1, p2_strict_val(&vec!["hgt",   "190cm"]));
        assert_eq!(0, p2_strict_val(&vec!["hgt", "190in"]));
        assert_eq!(0, p2_strict_val(&vec!["hgt", "190"]));
        assert_eq!(1, p2_strict_val(&vec!["hcl", "#123abc"]));
        assert_eq!(0, p2_strict_val(&vec!["hcl", "#123abz"]));
        assert_eq!(0, p2_strict_val(&vec!["hcl", "123abc"]));
        assert_eq!(1, p2_strict_val(&vec!["ecl",   "brn"]));
        assert_eq!(0, p2_strict_val(&vec!["ecl", "wat"]));
        assert_eq!(1, p2_strict_val(&vec!["pid",   "000000001"]));
        assert_eq!(0, p2_strict_val(&vec!["pid", "0123456789"]));
    }

    #[test]
    fn p2_data_parser() {

        let raw = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let res = vec![
            vec![
                vec!["ecl","gry"], vec!["pid","860033327"],vec!["eyr","2020"],vec!["hcl","#fffffd"],vec!["byr","1937"],vec!["iyr","2017"],vec!["cid","147"],vec!["hgt","183cm"]
            ],
            vec![
                vec!["iyr","2013"],vec!["ecl","amb"],vec!["cid","350"],vec!["eyr","2023"],vec!["pid","028048884"],vec!["hcl","#cfa07d"],vec!["byr","1929"]
            ],
            vec![
                vec!["hcl","#ae17e1"],vec!["iyr","2013"],vec!["eyr","2024"],vec!["ecl","brn"],vec!["pid","760753108"],vec!["byr","1931"],vec!["hgt","179cm"]
            ],
            vec![
                vec!["hcl","#cfa07d"],vec!["eyr","2025"],vec!["pid","166559648"],vec!["iyr","2011"],vec!["ecl","brn"],vec!["hgt","59in"]
            ]
        ];
        
        assert_eq!(res, p2_prepare_data(raw));
    }

    #[test]
    fn p2_valid_counter_invalid() {
        let _raw = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        // assert_eq!(0, _p2_run(raw));
    }


    #[test]
    fn p2_valid_counter_valid() {

        let _raw = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        

        // assert_eq!(4, _p2_run(raw));
    }

}





