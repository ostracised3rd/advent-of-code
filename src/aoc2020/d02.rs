use prse::parse;

pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let lines = data.lines()
            .map(|l| parse!(l, "{}-{} {}: {}"))
            .collect::<Vec<(usize, usize, char, &str)>>();

        let res = policies(lines, p1_policy);
        dbg!("{}", res);
    }
    
    fn p2(&self, data: String) {
        let lines = data.lines()
            .map(|l| parse!(l, "{}-{} {}: {}"))
            .collect::<Vec<(usize, usize, char, &str)>>();
    
        let res: u32 = policies(lines, p2_policy);
        dbg!("{}", res);
    }
}


fn p1_policy(low: usize, high: usize, policy: char, password: &str) -> bool {
    let len = password
        .chars()
        .filter(|x| *x == policy)
        .collect::<String>()
        .len();

    len >= low && len <= high
}


fn p2_policy(i: usize, j: usize, policy: char, password: &str) -> bool {
    let list: Vec<char> = password.chars().collect();
    (list[i-1] == policy) ^ (list[j-1] == policy)
}


fn policies(
    list: Vec<(usize, usize, char, &str)>, 
    func: fn(i: usize, j: usize, policy: char, password: &str)-> bool
) -> u32 {
    list
        .iter()
        .map(|x| if func(x.0, x.1, x.2, x.3) {1} else {0})
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy1() {
        assert!(p1_policy(1, 3, 'a', "abcde"));
        assert!(!p1_policy(1, 3, 'b', "cdefg"));
        assert!(p1_policy(2, 9, 'c', "ccccccccc"));
    }

    #[test]
    fn policy2() {
        assert!(p2_policy(1, 3, 'a', "abcde"));        
        assert!(!p2_policy(1, 3, 'b', "cdefg"));
        assert!(!p2_policy(2, 9, 'c', "ccccccccc"));
    }

    #[test]
    fn correct_password_policy() {
        let passwords = vec![
            (1, 3, 'a', "abcde"),
            (1, 3, 'b', "cdefg"),
            (2, 9, 'c', "ccccccccc"),
        ];

        assert_eq!(2, policies(passwords, p1_policy));
    }

    #[test]
    fn correct_actual_policy() {
        let passwords = vec![
            (1, 3, 'a', "abcde"),
            (1, 3, 'b', "cdefg"),
            (2, 9, 'c', "ccccccccc"),
        ];

        assert_eq!(1, policies(passwords, p2_policy));
    }
}