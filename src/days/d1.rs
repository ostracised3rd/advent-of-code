pub struct AoC;

impl super::Day for AoC {
    fn p1(&self, data: String) {
        let inputs = data
            .lines()
            .map(|x| x.trim().parse().unwrap())
            .collect();
    
        let res = p1_expense_report(inputs);
        println!("{}", res);
    }
    
    fn p2(&self, data: String) {
        let inputs = data
            .lines()
            .map(|x| x.trim().parse().unwrap())
            .collect();
    
        let res3 = p2_expense_report(inputs);
        println!("{}", res3);
    }
}


fn p1_expense_report(inputs: Vec<i32>) -> i32 {
    for i in inputs.iter() {
        for j in inputs.iter() {
            if i+j == 2020 {
                return i * j
            }
        }
    }

    0
} 

fn p2_expense_report(inputs: Vec<i32>) -> i32 {
    for i in inputs.iter() {
        for j in inputs.iter() {
            for h in inputs.iter() {
                if i+j+h == 2020 {
                    println!("{} {} {}", &i, &j, &h);
                    return i * j * h
                }
            }
        }
    }

    0
} 





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_report_check() {
        let inputs = vec![1721, 979, 366, 299, 675,1456];
        assert_eq!(514579, p1_expense_report(inputs));
    }

    #[test]
    fn p2_report_check() {
        let inputs = vec![1721, 979, 366, 299, 675,1456];
        assert_eq!(241861950, p2_expense_report(inputs));
    }
}