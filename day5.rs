// I also struggled very badly with this question. I think it's mainly just because I suck at looping, and it is 
// tough to iterate over things you want to change in Rust.

// Regardless, after a long struggle I got it. Again, I left my previous attempt in here just for posterity. I went
// the wrong way about it; I was getting correct answers, but without the loop this was a hopeless exercise.

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
    let mut badinstructionslist: Vec<Vec<i32>> = Vec::new();
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
                    // unpack the pages variable just to get the list
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
            } else {
                let mut cloneinstructions = instructions.clone();
                badinstructionslist.push(cloneinstructions);
            }
        }    
        println!("{:?}",answersum);
        // final list of pages to get the answer from
        let mut finalpages: Vec<Vec<i32>> = Vec::new();
        // second answer sum
        let mut secondanswersum: i32 = 0;
        let mut fullinstructionsgood = false;
        for badinstructions in &mut badinstructionslist{
            loop {
                    {
                    let mut i = 0;
                    let instructionsclone = badinstructions.clone();
                    let mut list_changed = false;
                    let mut fullinstructionsgood = true;
                    for instruction in &instructionsclone {
                        let mut pages = &pageorder.iter().find(|inner| inner.first() == Some(&instruction));
                        i+=1;
                        for deepinstruction in &instructionsclone[i..]{
                            if let Some(pagenooption) = pages{
                                if pagenooption.contains(deepinstruction){
                                } else {
                                    badinstructions.remove(badinstructions.iter().position(|x| x == deepinstruction).expect("Couldn't"));
                                    badinstructions.insert(i-1,*deepinstruction);
                                    list_changed = true;
                                    fullinstructionsgood = false;
                                }
                            }
                        }
                    }
                    if !list_changed{
                        break;
                    }
                    
                }
            }
            
            let gethalf = badinstructions.len();
            let middleslice = gethalf /2;
            secondanswersum += badinstructions[middleslice];
         }
         println!("{:?}",secondanswersum);
    }

        // fn rearragenlist(list){
        //     for instruction in list{
                
        //     }
        // }
        // iterate through the bad instruction list i made in the last question, same
        // method as the first. Need to find a way to modify the list when conditions
        // are not met, iterating over it again to check if it's good.
        // for badinstructions in &badinstructionslist{
        //     let mut i = 0;
        //     let mut fullinstructionsgood = true;
        //     let mut repairedlist: Vec<i32> = Vec::new();
        //     repairedlist.push(badinstructions[0]);
        //     // iterate over the instructions, finding the "pageset" for each page. add by 1 after to modify the for loop to check on instruction pages after current one.
        //     // this is how we know pages are good
        //     for instruction in badinstructions{
        //         let mut pages = (&pageorder.iter().find(|inner| inner.first() == Some (&instruction))).clone();
        //         i+=1;
        //         let mut j = 0;
        //         // once we have the page selected, check the rest of the pages after it to see if they pass.
        //         'inner: for deepinstruction in &badinstructions[i..]{
        //             j += 1;
        //             // unpack the pages variable just to get the list
        //             // deep instructions cannot be pushed; we can only push once the instruction for loop is 
        //             // done (all instructions are good for this page) or if a page is bad (we must insert)
        //             // at i-1.
        //             if let Some(pagenooption) = pages{
        //                 // if the list contains the next page, we are good.
        //                 if pagenooption.contains(deepinstruction){
        //                     fullinstructionsgood = true;
        //                 } else {
        //                     // fixing the list will always look the same.
        //                     if repairedlist.contains(&badinstructions[i+j-1]){
        //                     } else {
        //                     repairedlist.insert(i-1,badinstructions[i+j-1]);
        //                     fullinstructionsgood = false;
        //                     }
        //                 }

        //             }
        //         }
        //         // if the pages are in the right order, push the page to the list, but only if it's not already there.
        //         if fullinstructionsgood{
        //             if repairedlist.contains(instruction){
        //             } else {
        //             repairedlist.push(*instruction);
        //             }
        //         }
        //         // if it's wrong, push the page to the spot one from the right side.
        //         else {
        //             repairedlist.push(*instruction);
        //         }
        //     }
        //     finalpages.push(repairedlist.clone());
        //     repairedlist.clear();
        // }
        // println!("{:?}",finalpages);
    
     
    
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
