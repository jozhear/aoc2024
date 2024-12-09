use std::fs;

fn main() {
    let data = fs::read_to_string("day6.txt")
        .expect("Should have been able to read the file");
    // convert the data into a vector of vectors; makes for easier iteration.
    let newdata: Vec<Vec<String>> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string())
                .collect()
        })
        .collect();
    // define variables we'll be working with; a list of the spots guard has to turn,
    // coordinates of the guard and movements guard can take. 
    let mut obstaclevec: Vec<Vec<i32>> = Vec::new();
    let mut guardvec: Vec<i32> = Vec::new();
    let mut stepvec: Vec<Vec<i32>> = Vec::new();
    let movements = [[0,-1],[1,0],[0,1],[-1,0]];
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    // push the coordinates of obstacles and guard to appropriate vector.
    for row in &newdata{
        for column in row{
            if column == "#"{
                let mut coordvec: Vec<i32> = Vec::new();
                coordvec.push(x);
                coordvec.push(y);
                obstaclevec.push(coordvec.clone());
                coordvec.clear();
            } else if column == "^"{
                guardvec.push(x);
                guardvec.push(y);
            }
            x +=1;
        }
        x = 0;
        y +=1;
    }
    let rowlength = newdata[0].len();
    let columnlength = newdata.len();
    x = 0;
    y = 0;
    let mut z: usize = 0;
    // need to loop. this will go on for a while. guardmoving is a bool that lets us know when to stop.
    // we only ever check for z % 4 so that we don't have to worry about any complicated logic or 
    // going past the index of the movements var. Just have to put the movements in the right order (up is MINUS in this case)
    // if stepvec doesn't contain the guard's current position, push it. Then, count the length of step vec to get the answer.
    let mut guardmoving = true;
    while guardmoving {
        let nextstep: Vec<i32> = guardvec.iter().zip(movements[z % 4].iter()).map(|(&a,&b)| a+b).collect();
        if obstaclevec.contains(&nextstep){
            z +=1;
        } else if nextstep[0] < 0 || nextstep[0] > rowlength as i32 || nextstep[1] < 0 || nextstep[1] > columnlength as i32{
            guardmoving = false;
        }
        guardvec[0] = guardvec[0] + movements[z % 4][0];
        guardvec[1] = guardvec[1] + movements[z % 4][1];
        if !stepvec.contains(&guardvec){
            stepvec.push(guardvec.clone());
        }
    }
    println!("{:?}",stepvec.len());
}
