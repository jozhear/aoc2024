// 6a was so easy, but 6b was very hard. I had the right idea, but a few problems occured preventing me from getting right answer.
// It also takes a long time to run 6b. So I got the answer, but this is a prime candidate for cleanup in a few days.

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
    let mut secondstepvec: Vec<Vec<i32>> = Vec::new();
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
    let mut originalvec = guardvec.clone();
    let rowlength = newdata[0].len();
    let columnlength = newdata.len();
    x = 0;
    y = 0;
    let mut z: usize = 0;
    // need to loop. this will go on for a while
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
        guardvec.push((z % 4) as i32);
        secondstepvec.push(guardvec.clone());
        guardvec.pop();

    }
    z = 0;
    originalvec.push(z as i32);
    let resetvec = originalvec.clone();
    // B;
    let mut t = 0;
    let mut obstructions: i32 = 0;
    let mut newobstacledata = obstaclevec.clone();
    let mut newstepvec: Vec<Vec<i32>> = Vec::new();
    let mut oob = stepvec.pop();
    println!("{:?}",originalvec);
    // iterate across the steps from the first question. The guard cannot reach an obstacle not on our previous route, unless we add the obstacle.
    // There is no need to go across the ENTIRE board as the guard can't reach those places anyway.
    for piece in &stepvec{
        // Create a new obstacle, and push it to our list of known obstacles. Then, run the same logic as before.
        let mut createdobstacle: Vec<i32> = Vec::new();
        createdobstacle.push(piece[0]);
        createdobstacle.push(piece[1]);
        newobstacledata.push(createdobstacle.clone());
        let mut guardmoving = true;
        while guardmoving {
            // determine the coords of the next step.
            let mut nextstep: Vec<i32> = originalvec.iter().zip(movements[z % 4].iter()).map(|(&a,&b)| a+b).collect();
            // if you meet an obstacle, turn 90 degrees to the right. if you go out of bounds, break out of the while loop
            // and place the next obstacle in a different location.
            while newobstacledata.contains(&nextstep){
                z += 1;
                nextstep = originalvec.iter().zip(movements[z % 4].iter()).map(|(&a,&b)| a+b).collect();
            } if nextstep[0] < 0 || nextstep[0] > rowlength as i32 || nextstep[1] < 0 || nextstep[1] > columnlength as i32{
                newstepvec.clear();
                break
            } 
            // make the movement; append the piece to the vector of current steps for this round, along with the direction;
            // if those coordinates and that direction are on the list, break out of the loop, adding one to let mut i32 obstructions (our answer).
            originalvec[0] = originalvec[0] + movements[z % 4][0];
            originalvec[1] = originalvec[1] + movements[z % 4][1];
            originalvec[2] = (z % 4) as i32;
            if !newstepvec.contains(&originalvec){
                newstepvec.push(originalvec.clone());
            } else if newstepvec.contains(&originalvec) {
                obstructions += 1;
                newstepvec.clear();
                break;
            }
        }
        z = 0;
        newobstacledata.pop();
        createdobstacle.clear();
        originalvec = resetvec.clone();
    }
    stepvec.push(oob.expect("Couldn't add oob piece"));
    println!("{:?}",stepvec.len());
    println!("{:?}",obstructions);
}
