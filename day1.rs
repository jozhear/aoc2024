use std::fs;

fn main() {
    let data = fs::read_to_string("day1.txt");

    let mut leftlist: Vec<i32>  = Vec::new();
    let mut rightlist: Vec<i32> = Vec::new();
    match data {
        Ok(contents) => for row in contents.split("\r"){
        let rowvalues: Vec<&str> = row.split_whitespace().collect();
        leftlist.push(rowvalues[0].parse::<i32>().unwrap());
        rightlist.push(rowvalues[1].parse::<i32>().unwrap())
    },
        Err(error) => eprintln!("Error reading file: {error}"),
    }

    leftlist.sort();
    rightlist.sort();

    let mut i = 0;
    let mut sum = 0;

    for number in &leftlist{
        let mut difference = rightlist[i] - leftlist[i];
        if difference < 0 {
            difference = difference * -1;
        }
        i += 1;
        sum += difference;
    }
    println!("{:?}",sum)
}
