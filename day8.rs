use std::fs;
use std::collections::HashSet;

fn main() {
    // split into the lines, and split those lines into individual strings. 
    // collect them into a vector, and collect those vectors into the outer vector.
    let data = fs::read_to_string("day8.txt")
        .expect("Should have been able to read the file");
    let newdata: Vec<Vec<String>> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string())
                .collect()
        })
        .collect();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut actualpieces: Vec<Vec<i32>> = Vec::new();
    let mut answernodes: Vec<Vec<i32>> = Vec::new();
    let mut secondanswernodes: Vec<Vec<i32>> = Vec::new();
    for row in &newdata.clone(){
        for column in row{
            if column != "."{
                // if a piece does not equal ".", create a new vec. push the ordinal value, x, and y coords to the vec.
                // check the other antennaes of the same value to see if they exist, and if so, do a comparison to 
                // push the antinode to the answernodes outervec.
                if let Some(first) = column.chars().next(){
                    let ord = first as u32 - 48;
                    let mut antennaevec = vec![ord as i32,x,y];
                    // find if the outer vec contains an innervec of first value of same ord.
                    // if so, we can find for answernodes in this bracket instead of running another for loop.
                    // push the respective nodes to each answer from the 'deltas' fn.
                    let sameantennaes: Vec<&Vec<i32>> = actualpieces.iter().filter(|inner| inner.first() == Some(&antennaevec[0])).collect();
                    if sameantennaes.is_empty(){
                    } else {
                        for antennae in sameantennaes{
                            let (mut currentanswernodes,mut currentsecondanswernodes) = deltas(antennae,&antennaevec,row,newdata.clone(),ord as i32);
                            answernodes.extend(currentanswernodes.clone());
                            secondanswernodes.extend(currentsecondanswernodes.clone());
                            let (mut currentanswernodes,mut currentsecondanswernodes) = deltas(&antennaevec,antennae,row,newdata.clone(),ord as i32);
                            answernodes.extend(currentanswernodes.clone());
                            secondanswernodes.extend(currentsecondanswernodes.clone());
                        }
                    }
                // this needs to stay here. this is how we check against other antennae of the same frequency.
                actualpieces.push(antennaevec.clone());
            }
            }
            x+=1
        }
        x = 0 ;
        y+=1
    }
    // these two functions are how we filter out duplicates. They do not search for the ordinal value of each
    // antennae (a frequency represented by a number instead of string.) They only check for coordinates.
    // If there are duplicate values, we retain only the first instance of each.
    let mut seen = HashSet::new();
    answernodes.retain(|inner| {
        seen.insert((inner[inner.len() - 2], inner[inner.len() - 1]))
    });
    println!("{:?}",answernodes.len());
    let mut seen = HashSet::new();
    secondanswernodes.retain(|inner| {
        seen.insert((inner[inner.len() - 2], inner[inner.len() - 1]))
    });
    println!("{:?}", secondanswernodes.len());
}

fn deltas(antennae1: &Vec<i32>,antennae2: &Vec<i32>,row: &Vec<String>,newdata: Vec<Vec<String>>,ord: i32) -> (Vec<Vec<i32>>,Vec<Vec<i32>>){
    // this can probably be cleaned up. We find the xcoorddiff and ycoorddiff (delta) of the compared pieces, calculating x and y coords
    // of the piece we are going to look at. we check for boundaries, and if that passes, we push the node to answernodes, and the 
    // node along with the comparison nodes to secondanswernodes (since it makes a line of at least 2 from all segments).
    // we then run a while loop using the inbounds variable to continue moving across the board until we are out of bounds,
    // pushing successful pieces to secondanswernodes. Returning these variables we exit the function.
    let xcoorddiff = antennae1[1] - antennae2[1];
    let ycoorddiff = antennae1[2] - antennae2[2];
    let xcoord = antennae1[1] + antennae1[1] - antennae2[1];
    let ycoord = antennae1[2] + antennae1[2] - antennae2[2];
    let mut inbounds = true;
    let mut answernodes: Vec<Vec<i32>> = Vec::new();
    let mut secondanswernodes: Vec<Vec<i32>> = Vec::new();
    if xcoord >= 0 && xcoord <= row.len()as i32 -1 && ycoord >= 0 && ycoord <= newdata.len() as i32 -1 {
        let mut antinode = vec![ord as i32,xcoord,ycoord];
        answernodes.push(antinode.clone());
        let mut loopclone = antinode.clone();
        antinode.clear();
        secondanswernodes.push(loopclone.clone());
        secondanswernodes.push(antennae1.clone());
        secondanswernodes.push(antennae2.clone());
        while inbounds{
            let mut furtherpiece: Vec<i32> = Vec::new();
            loopclone[1] = loopclone[1] + xcoorddiff;
            loopclone[2] = loopclone[2] + ycoorddiff;
            if loopclone[1] < 0 || loopclone[1] > row.len()as i32 -1 || loopclone[2] < 0 || loopclone[2] > row.len()as i32 -1{
                inbounds = false;
            } else {
                    secondanswernodes.push(loopclone.clone());
            }
    }
    }
    (answernodes,secondanswernodes)
}
