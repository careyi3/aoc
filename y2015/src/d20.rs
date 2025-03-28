use utils::harness::Solve;

pub struct D20;

impl Solve for D20 {
    fn part1(_input: String, _path: &String) -> String {
        let target = 36000000;
        let ans;
        let mut i = 1;
        loop {
            let mut sum = 0;
            let sqrt = f64::sqrt(i as f64).floor() as i32;
            for n in 1..sqrt + 1 {
                let rem = i % n;
                let quo = i / n;
                if rem == 0 {
                    sum += quo;
                    sum += n;
                }
            }
            if sum * 10 >= target {
                ans = i;
                break;
            }
            i += 1;
        }

        return ans.to_string();
    }

    fn part2(_input: String, _path: &String) -> String {
        let target = 36000000;
        let ans;
        let mut i = 1;
        loop {
            let mut sum = 0;
            let sqrt = f64::sqrt(i as f64).floor() as i32;
            for n in 1..sqrt + 1 {
                let rem = i % n;
                let quo = i / n;
                if rem == 0 {
                    if i / quo <= 50 {
                        sum += quo;
                    }
                    if i / n <= 50 {
                        sum += n;
                    }
                }
            }
            if sum * 11 >= target {
                ans = i;
                break;
            }
            i += 1;
        }

        return ans.to_string();
    }
}
