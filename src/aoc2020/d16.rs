pub struct AoC;
// TODO: solve !!!!
impl advent_of_rust::Day for AoC {
    fn p1(&self, _data: String) {
        todo!();
    }
    
    fn p2(&self, _data: String) {
        todo!();
    }
}


fn rule_parser(data: &str)  {
    // HashMap<&str, Vec<i32>>
    data.lines()
        .map(|x| x.split(": ").collect::<Vec<&str>>());

}

fn ticket_parser(data: &str) -> Vec<i32> {
    data.split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn ticket_separator(data: &str) -> Vec<Vec<i32>> {
    data.lines()
        .map(|x| ticket_parser(x))
        .collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid() {
        let rules = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50";

        let your_ticket =
"7,1,14";

        let nearby_tickets ="7,3,47
40,4,50
55,2,20
38,6,12";
    }

}





