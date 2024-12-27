use std::fs;

fn main() {
    let data = fs::read_to_string("day10.txt").expect("Couldn't open it");
    // define x and y coordinates
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let directions = [[1,0],[0,1],[-1,0],[0,-1]];
    let mut answersum: i32 = 0;
    let mut secondanswersum: i32 = 0;
    let newdata: Vec<Vec<i32>> = data
        .lines()
        .map(|line| {
            line.chars().map(|ch| {
                ch.to_string().parse::<i32>().expect("Couldn't parse it")
            })
            .collect()
        })
        .collect();
    let totalrows: i32 = newdata.len().try_into().unwrap();
    let totalcolumns: i32 = newdata[0].len().try_into().unwrap();
    for row in &newdata {
        for mut trailhead in row {
            if *trailhead == 0 {
                let mut queue: Vec<Vec<i32>> = Vec::new();
                let mut coords: Vec<i32> = Vec::new();
                coords.push(x);
                coords.push(y);
                let testcoords = coords.clone();
                queue.push(coords);
                let mut ninesteps: Vec<Vec<i32>> = Vec::new();
                while queue.len() > 0 {
                    let currentstep = queue.remove(0);
                    for direction in directions {
                    // iterate through the list of directions and determine next step
                        let nextstep: Vec<i32> = currentstep.iter().zip(direction.iter()).map(|(&a, &b)|a+b).collect();
                        if nextstep[0] > -1 && nextstep[1] > - 1 && nextstep[0] < totalcolumns as i32 && nextstep[1] < totalrows {
                            if newdata[nextstep[1] as usize][nextstep[0] as usize] - newdata[currentstep[1] as usize][currentstep[0] as usize] == 1 {
                                if newdata[nextstep[1] as usize][nextstep[0] as usize] != 9{
                                    queue.push(nextstep);
                                } else {
                                    if ninesteps.contains(&nextstep) {

                                    } else {
                                        ninesteps.push(nextstep);
                                        answersum += 1;
                                    }
                                    secondanswersum += 1;
                                }
                                
                            } else {
                            continue;
                        }
                        } else {
                        
                        }
                    }
            }
            }x +=1;
        } x = 0;
        y+=1;
    } 
    println!("{:?}",answersum);
    println!("{:?}",secondanswersum);
}
