use std::fs;

fn main() {
    let data = fs::read_to_string("day2.txt");

    let mut allrows: Vec<Vec<i32>> = Vec::new();
    let mut rowints: Vec<i32>  = Vec::new();
    let mut i = 0;
    let mut safereports = 0;
    match data {
        Ok(contents) => for row in contents.split("\r"){
        let rowvalues: Vec<&str> = row.split_whitespace().collect();
        for row in &rowvalues{
            let newrow = row.parse::<i32>().unwrap();
            rowints.push(newrow);
        }
        allrows.push(rowints.clone());
        rowints.clear();
    },
    Err(error) => eprintln!("Error reading file: {error}"),
    }
    for row in allrows{
        let mut sortedrow = row.clone();
        sortedrow.sort();
        if *row == sortedrow{
            for entry in &row[0..row.len() - 1]{
                if entry - &row[i+1] >= -3 && entry - &row[i+1] <= -1{
                    i+=1;
                }
                else {
                    break;
                }
                if i + 1 == row.len(){
                    safereports+=1;
                }
            }
        }
        sortedrow.reverse();
        if *row == sortedrow{
            for entry in &row[..row.len() - 1]{
                if entry - row[i+1] <= 3 && entry - row[i+1] >= 1{
                    i+=1;
                }
                else {
                    break;
                }
                if i + 1 == row.len(){
                    safereports+=1;
                }
            }
        }
        i = 0;
    }
    println!{"{:?}",safereports}
}
