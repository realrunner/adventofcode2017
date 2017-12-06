use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;


fn main() {
    part1();
    part2();
}

fn part1() {
    let f = File::open("data/day2.txt").unwrap();
    let file = BufReader::new(&f);
    let mut sum = 0;
    for line in file.lines() {
        let l = line.unwrap();
        let mut nums: Vec<i32> = l.split("\t")
            .map(|num_str| num_str.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        nums.sort();
        let min = nums.first().unwrap();
        let max = nums.last().unwrap();
        let diff = max - min;
        sum += diff;
    }
    println!("Sum of differences: {}", sum);
}

fn part2() {
    let f = File::open("data/day2.txt").unwrap();
    let file = BufReader::new(&f);
    let mut sum = 0;
    for line in file.lines() {
        let l = line.unwrap();
        let mut nums: Vec<i32> = l.split("\t")
            .map(|num_str| num_str.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        nums.sort();
        for x in nums.iter().rev() {
            for y in nums.iter().rev() {
                if x == y {
                    continue;
                }
                if x % y == 0 {
                    sum += x / y;
                }
            }
        }
    }
    println!("Sum of evenly divided numbers: {}", sum);
}