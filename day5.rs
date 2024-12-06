// included my FIRST attempt at the bottom. I assumed the pages had the transitive property.... Nope. Only realized my error when I thought I was about to get my answer.

use std::fs;

fn main() {
    let data = fs::read_to_string("day5.txt")
        .expect("Should have been able to read the file");
    // Split new data into lines, then enum over lines with map; if map contains
    // '|', split the line at the '|', convert the split out put (two str) to int
    // and collect back into the subarray. Do the same for lines containing ','.
    // Collect the subarrays back into the main arry to meet the Vec<Vec<i32>> type requirement.
    let newdata: Vec<Vec<i32>> = data
        .lines() // Split into lines
        .map(|line| {
            if line.contains('|'){
               line.split('|')
               .map(|num| num.trim().parse::<i32>().unwrap_or(0))
                .collect()
            } else if line.contains(','){
                line.split(',')
                .map(|num| num.trim().parse::<i32>().unwrap_or(0))
                .collect()
            } else {
                vec![]
            }
        })
        .collect();
    // println!("{:?}",newdata);
    let emptypage: usize = newdata.iter().position(|x| x.is_empty()).expect("couldn't find it");
    let mut pageorder: Vec<Vec<i32>> = Vec::new();
    let mut answersum: i32 = 0;
    for pageset in &newdata[0..emptypage]{
        if let Some(index) = pageorder.iter().position(|inner| inner.first() == Some(&pageset[0])){
            pageorder[index].push(pageset[1]);
        } else {
            let mut newpagevector: Vec<i32> = Vec::new();
            newpagevector.push(pageset[0]);
            newpagevector.push(pageset[1]);
            pageorder.push(newpagevector);
        }
        }
        for instructions in &newdata[emptypage+1..]{
            let mut i = 0;
            let mut fullinstructionsgood = true;
            'inner: for instruction in instructions{
                let pages = &pageorder.iter().find(|inner| inner.first() == Some(&instruction));
                i+=1;
                for deepinstruction in &instructions[i..]{
                    if let Some(pagenooption) = pages{
                        if pagenooption.contains(deepinstruction){
                    } else {
                        fullinstructionsgood = false;
                        break 'inner
                        };
                    }
                }
            }
            if fullinstructionsgood {
                let gethalf = instructions.len();
                let middleslice = gethalf /2;
                answersum += instructions[middleslice];
            }
        }    
        println!("{:?}",answersum);
    }
        
    
    // for pageset in &newdata[0..emptypage]{
    //     if pageorder.contains()
    // }
    // if newdata.contains(""){
    //     println!{"{:?}","Yes"}
    // }
    // for pageset in &newdata[0..emptypage]{
    //     if pageorder.contains(&pageset[0]){
    //         if pageorder.contains(&pageset[1]){
    //             let firstpageindex = pageorder.iter().position(|x| x == &pageset[0]);
    //             let secondpageindex = pageorder.iter().position(|x| x == &pageset[1]);
    //             if firstpageindex <  secondpageindex {
                    
    //             }
    //             else {
    //                 let pagemove = pageorder.remove(firstpageindex.expect("Couldn't take it"));
    //                 pageorder.insert(secondpageindex.expect("Couldn't push it"),pageset[0]);
    //             }
    //         }
    //         else{
    //             pageorder.push(pageset[1]);
    //         }
    //     } else if pageorder.contains(&pageset[1]){
    //         pageorder.insert(pageorder.iter().position(|x| x == &pageset[1]).expect("Couldn't push it"),pageset[0])
    //     }
    //     else {
    //         pageorder.push(pageset[0]);
    //         pageorder.push(pageset[1]);
    //     }
    // }


//     for instructionsets in &newdata[emptypage+1..]{
//         let mut correctorder: Vec<i32> = Vec::new();
//         for instructions in instructionsets{
//             correctorder.push(pageorder.iter().position(|x| x == instructions).expect("Couldn't push it").try_into().unwrap());
//         }
//         let mut cloneorder = correctorder.clone();
//         cloneorder.sort();
//         if correctorder == cloneorder{
//             println!("{:?}","yes");
//         }
//     }
//     println!("{:?}",pageorder);
 
