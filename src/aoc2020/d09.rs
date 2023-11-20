pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let data = data.lines()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let preamble = 25;
        let res = seek_n_find(preamble, &data);
        println!("{}", res);
    }
    
    fn p2(&self, data: String) {
        let data = data.lines()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let poi = 90433990;
        let res = seek_n_destroy(poi, &data);
        println!("{}", res);
    }
}

fn odd_num_out(left: usize, right: usize, sus: &u64, list: &Vec<u64>) -> bool {

    for i in left..right {
        let x = &list[i];
        for y in &list[i+1..right] {
            if x + y == *sus {
                return true
            }
        }
    }


    false
}


fn seek_n_find(preamble: usize, list: &Vec<u64>) -> u64 {

    for (i, sus) in list.iter().skip(preamble).enumerate() {
        if !odd_num_out(i, i + preamble, sus, list) {
            return *sus
        }
    }

    return 0
}


fn seek_n_destroy(poi: u64, list: &Vec<u64>) -> u64 {

    for (i, left) in list.iter().enumerate() {
        let mut res = 0;
        let mut sm = left;
        let mut lg = left;
        res += left;
        for right in &list[i+1..] {
            res += right;
            if res > poi {continue;}

            sm = sm.min(right);
            lg = lg.max(right);
            if res == poi {return sm + lg}
        }
    }

    return 0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finder() {
        let data = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 
            117, 150, 182, 127, 219, 299, 277, 309, 576,
        ];

        let preamble: usize = 5;
        let res = 127;

        assert_eq!(res, seek_n_find(preamble, &data));
    }

    #[test]
    fn destroyer() {
        let data = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 
            117, 150, 182, 127, 219, 299, 277, 309, 576,
        ];

        let poi = 127;

        assert_eq!(62, seek_n_destroy(127, &data));
    }
}





