use std::fs;
use regex::Regex;

fn main() {
    // extract the data into a string
    let data = fs::read_to_string("day3.txt")
        .expect("Should have been able to read the file");
    // define the regex looking for mul(x,y)
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let leftproduct = Regex::new(r"\d+").unwrap();
    // find the regex within the larger string, appending to a vector of strings.
    let products: Vec<&str> = re.find_iter(&data).map(|m| m.as_str()).collect();
    let mut answer: i32 = 0;
    for product in products{
        let numbers: Vec<&str> = leftproduct.find_iter(&product).map(|m| m.as_str()).collect();
        let leftnumber = numbers[0].parse::<i32>().unwrap();
        let rightnumber = numbers[1].parse::<i32>().unwrap();
        let rowanswer = leftnumber * rightnumber;
        answer += rowanswer;
    }
    println!("{:?}",answer);
}
