// Not proud of this one lol.
// Trying to keep up to one per day using a language I only learned a week ago, and kinda tough.
// I mean, it works, my logic was sound... but there is 100% a better way of doing this.

// Ah well, going to keep learning!!

// 20241203; truncated the code to only do one check on the if rows are sorted or not instead of two separate. 
// next order of business is to fix the for loop in the second part answer.

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
    for row in &allrows{
        let mut sortedrow = row.clone();
        sortedrow.sort();
        let mut reverserow = sortedrow.clone();
        reverserow.reverse();
        if *row == sortedrow || *row == reverserow{
            for entry in &row[..row.len() - 1]{
                if entry - &row[i+1] >= -3 && entry - &row[i+1] <= -1 || entry - &row[i+1] <=3 && entry - &row[i+1] >= 1{
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
    let mut z = 0;
    let mut newreports = 0;
    for row in &allrows{
        z = 0;
        'outer: for report in row{
            i = 0;
            let mut duperow = row.clone();
            let piece = duperow.remove(z);
            let mut sortedrow = duperow.clone();
            sortedrow.sort();
            let mut reverserow = sortedrow.clone();
            reverserow.reverse();
            if *duperow == sortedrow || *duperow == reverserow{
                'inner: for entry in &duperow[..duperow.len() - 1]{
                    if entry - &duperow[i+1] >= -3 && entry - &duperow[i+1] <= -1 || entry - &duperow[i+1] <=3 && entry - &duperow[i+1] >= 1{
                        i+=1;
                    }
                    else {
                        break 'inner;
                    }
                    if i + 1== duperow.len(){
                        newreports+=1;
                        break 'outer;
                    }
                }
            }
            z+=1;
        }      
    }
    println!{"{:?}",safereports};
    println!{"{:?}",newreports};
}

