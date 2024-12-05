use std::fs;

fn main() {
    let data = fs::read_to_string("day4.txt")
        .expect("Should have been able to read the file");
    let rows: Vec<Vec<String>> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string())
                .collect()
        })
        .collect();

    let movements = [[1,0],[1,-1],[0,-1],[-1,-1],[-1,0],[-1,1],[0,1],[1,1]];
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut z: usize = 0;
    let mut answersum = 0;
    let letters = ["X","M","A","S"];
    for row in &rows{
        let rowlength = (row.len() -1);
        let rowslength = (rows.len() -1);
        for character in row{
            if character == letters[z]{
                for movement in &movements{
                    let (mut nextstepx, mut nextstepy) = (
                        x as isize + movement[0] as isize,
                        y as isize + movement[1] as isize
                        
                    );
                    z +=1;
                    if nextstepx >= 0 && nextstepx <= rowlength.try_into().unwrap() && nextstepy >=0 && nextstepy <= rowlength.try_into().unwrap(){
                        if rows[nextstepy as usize][nextstepx as usize] == letters[z]{
                            let (mut thirdstepx, mut thirdstepy) = (
                                nextstepx as isize + movement[0] as isize,
                                nextstepy as isize + movement[1] as isize
                            );
                            z += 1;
                            if thirdstepx >= 0 && thirdstepx <= rowlength.try_into().unwrap() && thirdstepy >= 0 && thirdstepy <= rowlength.try_into().unwrap(){
                                if rows[thirdstepy as usize][thirdstepx as usize] == letters[z]{
                                    let (mut finalstepx, mut finalstepy) = (
                                        thirdstepx as isize + movement[0] as isize,
                                        thirdstepy as isize + movement[1] as isize
                                    );
                                    z += 1;
                                    if finalstepx >= 0 && finalstepx <= rowlength.try_into().unwrap() && finalstepy >= 0 && finalstepy <= rowlength.try_into().unwrap(){
                                        if rows[finalstepy as usize][finalstepx as usize] == letters[z]{
                                            answersum += 1;
                                        }
                                    }
                            }
                            
                            }
                        }
                    }
                    z = 0;
                }
            }
            x +=1;
        }
        y+=1;
        x = 0;
    }
    println!("{:?}",answersum);
    // y and x are 1 because A is always 1 away from edges for this to work
    y = 1;
    x = 1;
    let mut secondanswer: i32 = 0;
    // same reason as above, not interested in the edge rows or columns
    for row in &rows[1..(&rows.len() -1)]{
        for character in &row[1..(&rows.len() -1)]{
            // find only A
            if character == "A"{
                // define the surrounding characters, making them into a list
                let mut cornersteps = [
                    rows[(y-1) as usize][(x+1) as usize].clone(),
                    rows[(y-1) as usize][(x-1) as usize].clone(),
                    rows[(y+1) as usize][(x-1) as usize].clone(),
                    rows[(y+1) as usize][(x+1) as usize].clone()
                    ];
                    println!("{:?}",cornersteps);
                    if cornersteps == ["M","M","S","S"]{
                        
                        secondanswer+=1;
                    } 
                    else if cornersteps == ["S","S","M","M"]{
                        secondanswer+=1;
                    }
                    else if cornersteps == ["M","S","S","M"]{
                        secondanswer+=1;
                    }
                    else if cornersteps == ["S","M","M","S"]{
                        secondanswer+=1;
                    }
                }
                x+=1;
            }
            y+=1;
            x = 1;
        }
    println!("{:?}",secondanswer);
    }
