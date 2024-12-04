use std::fs;
use regex::Regex;

fn main() {
    // extract the data into a string
    let data = fs::read_to_string("day3.txt")
        .expect("Should have been able to read the file");
    // define the regex looking for mul(x,y)
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    // second re is for part 2. This regex combines what's needed for part 1 as well as the do() and don't().
    let secondre = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don\'t\(\)").unwrap();
    let leftproduct = Regex::new(r"\d+").unwrap();
    // find the regex within the larger string, appending to a vector of strings.
    let products: Vec<&str> = re.find_iter(&data).map(|m| m.as_str()).collect();
    let mut answer: i32 = 0;
    // run the for loop, creating vectors of the two numbers. then, turn them each to int, and multiply, adding to the final answer.
    for product in products{
        let numbers: Vec<&str> = leftproduct.find_iter(&product).map(|m| m.as_str()).collect();
        let leftnumber = numbers[0].parse::<i32>().unwrap();
        let rightnumber = numbers[1].parse::<i32>().unwrap();
        let rowanswer = leftnumber * rightnumber;
        answer += rowanswer;
    }
    // multiplybit is basicaly just a boolean. if the previous "bit" was don't(), it's 0; otherwise, it's 1. As long as it's 1, follow the same logic as last time.
    let mut multiplybit = 1;
    let secondanswer: Vec<&str> = secondre.find_iter(&data).map(|m| m.as_str()).collect();
    let mut secondanswerfinal: i32 = 0;
    for entry in &secondanswer{
        if *entry == "don't()"{
            multiplybit = 0;
        }
        else if * entry == "do()"{
            multiplybit = 1;
        }
        else{
            if multiplybit == 1{
                let numbers: Vec<&str> = leftproduct.find_iter(&entry).map(|m| m.as_str()).collect();
                let leftnumber = numbers[0].parse::<i32>().unwrap();
                let rightnumber = numbers[1].parse::<i32>().unwrap();
                let rowanswer = leftnumber * rightnumber;
                secondanswerfinal += rowanswer;
            }
        }
    }
    println!("{:?}",answer);
    println!("{:?}",secondanswerfinal);
}
