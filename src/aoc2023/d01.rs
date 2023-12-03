pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {

        // let _inputs = data
        //     .lines()
        //     .map(|x| x.trim().parse().unwrap())
        //     .collect();
    
        // let res = p1_expense_report(inputs);
        // println!("{}", res);
    }
    
    fn p2(&self, data: String) {
        // let _inputs = data
        //     .lines()
        //     .map(|x| x.trim().parse().unwrap())
        //     .collect();
    
        // let res3 = p2_expense_report(inputs);
        // println!("{}", res3);
    }
}


mod tests {
    use super::*;

    #[test]
    fn map_creation() {

        let text = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let res = vec![
            vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0], 
            vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0], 
            vec![0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0], 
            vec![0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1], 
            vec![0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0], 
            vec![0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0], 
            vec![0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1], 
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
            vec![1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0], 
            vec![1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1], 
            vec![0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1]
        ];


        // assert_eq!(res, prepare_map(text));
    }


}