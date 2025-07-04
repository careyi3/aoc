use utils::{file_reader, harness::Solve};

pub struct D10;

impl Solve for D10 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let lengths: Vec<usize> = input
            .first()
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        let mut nums: Vec<usize> = (0..=255).collect();

        let mut position = 0;
        let mut skip = 0;
        for length in lengths {
            let target = length / 2;
            for i in 0..target {
                let nums_len = nums.len();
                let position_one = (position + i) % nums_len;
                let position_two = ((position + length - 1) - i) % nums_len;
                let one = nums[position_one];
                let two = nums[position_two];
                nums[position_two] = one;
                nums[position_one] = two;
            }
            position = (position + skip + length) % nums.len();
            skip += 1;
        }

        let ans1 = nums[0];
        let ans2 = nums[1];

        return (ans1 * ans2).to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let chars: Vec<char> = input.first().unwrap().chars().collect();

        let mut lengths: Vec<usize> = vec![];
        for char in chars {
            lengths.push(char as usize);
        }
        let mut suffix: Vec<usize> = vec![17, 31, 73, 47, 23];
        lengths.append(&mut suffix);

        let mut nums: Vec<usize> = (0..=255).collect();

        let mut position = 0;
        let mut skip = 0;
        for _ in 0..64 {
            for length in &lengths {
                let target = length / 2;
                for i in 0..target {
                    let nums_len = nums.len();
                    let position_one = (position + i) % nums_len;
                    let position_two = ((position + length - 1) - i) % nums_len;
                    let one = nums[position_one];
                    let two = nums[position_two];
                    nums[position_two] = one;
                    nums[position_one] = two;
                }
                position = (position + skip + length) % nums.len();
                skip += 1;
            }
        }

        let mut answer: Vec<u8> = vec![];
        let mut xor: u8 = 0;
        for (idx, num) in nums.iter().enumerate() {
            if idx == 0 {
                xor = *num as u8;
                continue;
            }
            if idx % 16 == 0 {
                answer.push(xor);
                xor = *num as u8;
            } else {
                xor = xor ^ *num as u8
            }
        }
        answer.push(xor);

        let mut answer_string = "".to_string();
        for num in answer {
            answer_string.push_str(&format!("{:02x}", num));
        }

        return answer_string;
    }
}
