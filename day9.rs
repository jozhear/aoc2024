use std::fs;

fn main() {
    let data = fs::read_to_string("day9.txt").expect("Couldn't read it");
    let newdata: Vec<String> = data.chars().map(|ch| ch.to_string()).collect();
    // blockid represents the int id of the block. it goes up when enough blocks of space have been written.
    let mut blockid = 0;
    // end piece starts from the end and works towards the beginning. this is what spares us from a second for loop; we just go backwards, incrementing when the conditions (moves variable in the 'inner 
    // for loop) meet.
    let mut endpiece = newdata.len() - 1;
    // we define this here and now so that we can keep our logic separate from the for loop. this is so we don't have to define every time we determine its time to start adding from the end. 
    let mut moves = newdata[endpiece].parse::<i128>().expect("couldn't do it");
    // answersting is the final loop. easy to determine answer once we have constructed a list of block id's and their size.
    let mut answerstring: Vec<String> = Vec::new();
    let mut answersum = 0;
    let mut secondanswersum = 0;
    'inner: for blocks in &newdata {
         // the for loop has to stop when you have reached a point in the for loop that you have already evaluated with the endpieces var.
         // once that point comes, you know you've done the whole (moved it all over), it has to end now.
        if (blockid/2) == (endpiece/2) {
            // however, you still need to check if you have pending moves, as that might mean there are blocks remaining to be counted
            // that don't need to be moved. If you don't do this your answer will be too small if there are remaining moves.
            for _movement in 0..moves{
             answerstring.push((blockid/2).to_string());   
            }
             break 'inner;
         } else {
         // iterate through the current block. if it can %2, we push ( a block from the left). if not, we can take from the right since this is free space.
         let turns = blocks.parse::<i128>().expect("Couldn't do it");
          for _turn in 0..turns{
             if blockid % 2 == 0 {
                 answerstring.push((blockid/2).to_string());
             } else {
                 // if the move value is 0, segue past it; the while loop is what makes this work. We don't want to waste a turn on a 0.
                 while moves == 0 {
                     endpiece -= 2;
                     moves = newdata[endpiece].parse::<i128>().expect("couldn't do it");
                 } 
                 // keep pushing until moves is 0, and then move on to the next one.
                 if moves > 0 {
                     // answer b logic will reside here
                     
                         answerstring.push((endpiece/2).to_string());
                         moves -=1;
                 } else {
                    moves = newdata[endpiece].parse::<i128>().expect("couldn't do it");
                    endpiece -= 2;
                 } 
             }
         }
        }
         // always add; blockid goes up by 1 even if it's free space, but when we divide by 2 we get what we want for block id regardless.
         blockid += 1;
     }
     // start from 0 again. second endpiece is our main anchor, however we are going backwards this time; the outer loop will end when we get all the way to the end on the left.
    blockid = 0;
    let mut secondendpiece = newdata.len() -1;
    let mut secondanswerdata = newdata.clone();
    let mut index = 0;
    // go backwards in steps of 2.
    'outer: for blocks in newdata.clone().into_iter().rev().step_by(2) {
        // go forwards in steps of 1, but check if index is % 2 or not. If it isn't, add the sum of the pieces we've gone over by 1.
        'inner: for turns in 0..secondendpiece {
            if turns % 2 == 0{
                index += secondanswerdata[turns].parse::<i32>().expect("Couldn't do it");
            } else {
                // modify the list to make sure on the next iteration we are checking an updated one; add the pieces we've moved to where they go (making a contiguous block) and make the gap smaller (in case a gap gets two or more moves)
                if blocks.parse::<i32>().expect("couldn't do it") <= secondanswerdata[turns].parse::<i32>().expect("Couldn't do it") {
                    secondanswerdata[turns] = ((secondanswerdata[turns].parse::<i32>().expect("Couldn't do it")) - blocks.parse::<i32>().expect("Couldn't do it")).to_string();
                    secondanswerdata[turns - 1] = ((secondanswerdata[turns- 1].parse::<i32>().expect("Couldn't do it")) + blocks.parse::<i32>().expect("Couldn't do it")).to_string();
                    secondanswerdata[secondendpiece] = ((secondanswerdata[secondendpiece].parse::<i32>().expect("Couldn't do it")) - blocks.parse::<i32>().expect("Couldn't do it")).to_string();
                    secondanswerdata[secondendpiece - 1] = ((secondanswerdata[secondendpiece - 1].parse::<i32>().expect("Couldn't do it")) + blocks.parse::<i32>().expect("Couldn't do it")).to_string();
                    // when we find a place we can move, calculate the sum towards the answer. we don't have to bother with it again this way
                    for _turn in 0..blocks.parse::<i32>().expect("Couldn't do it"){
                        secondanswersum += secondendpiece / 2 * index as usize;
                        index +=1;
                    }
                    break 'inner;
                } else { 
                    index += secondanswerdata[turns].parse::<i32>().expect("Couldn't do it");
                }
                if turns == secondendpiece - 1 {
                    // if the piece cannot move, calculate the answer toward the sum. iterate over the number of blocks, appending 1 to idnex each time.
                    for _turn in 0..blocks.parse::<i32>().expect("Couldn't do it"){
                        secondanswersum += secondendpiece / 2 * index as usize;
                        index +=1;
                    }
                }
            }
        }
        // stop the for loop when it's reached the end.
        if secondendpiece > 1 {
            secondendpiece -= 2;
        } index = 0;
     }
    //  }
         //blockid += 1;
    // enumerate through the vec of strings, multiplying the index (field) by the blocksize as an i32. 
    // keep adding, and this is what gives us our answer.
    for (fileid,blocksize) in answerstring.iter().enumerate() {
        answersum += fileid as i128 * blocksize.parse::<i128>().expect("Couldn't do it");
    }
    println!("{:?}",answersum);
    println!("{:?}",secondanswersum);
}
