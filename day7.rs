use std::fs;
// split the data into a vec of vecs. we want the sum, or product
// in position[0] with the rest afterward.

// this is done by splitting the rows into lines, then performing
// a function on each line, first splitting by ':', then pushing the converted
// value of type i128 (if it exists) to a newly created numvector vector.

// then, if the rest of the numbers exist, we push all of them (extend) to the vector,
// trimming whitespace and handling for errors in unwrap_or(0).

// we then call numvector to return the vector to be collected to the outer vector in .collect();
fn main() {
    let mut data = fs::read_to_string("day7.txt").expect("Couldn't");
    let mut newdata: Vec<Vec<i128>> = data
    .lines()
    .map(|line| {
        let mut numbers = line.split(':');
        let mut numvector: Vec<i128> = Vec::new();
        if let Some(first) = numbers.next() {
            numvector.push(first.trim().parse::<i128>().unwrap_or(0));
        }
        if let Some(rest) = numbers.next(){
            numvector.extend(
                rest.split_whitespace()
                    .map(|num| num.trim().parse::<i128>().unwrap_or(0)),
            );
        }
        numvector
    })
    .collect();
    // for each row in the outer vector, run our operation. we need to add the number in position[0]
    // to answersum if any of the operations prove to be true. we do this in the check_numbers
    // function by returning answertotal = numbers[0] if runningtotal ever equals numbers[0].
    let mut answersum: i128 = 0;
    let mut secondanswersum: i128 = 0;
    for numbers in newdata{
        // operation count is how we get the different answers. we bind our functions to variable 'operations'
        // and check for remainder over 2 (or 3 in question B) to "choose" the next operation. finding total
        // number of operations ^ length of the numbers - 1 lets us get a number we can iterate over, essentially 
        // giving us all possible combinations of */+ and for B */+/concat 
       let mut operationCount = 2;
       answersum += check_numbers(numbers.clone(),operationCount);
       let mut operationCount = 3;
       secondanswersum += check_numbers(numbers.clone(),3);
    }
    println!("{:?}",answersum);
    println!("{:?}",secondanswersum);
}

fn multiply(a: i128,b: i128) -> i128 {
    return a + b
}

fn add(a: i128,b: i128) -> i128 {
    return a * b
}

fn concatenate(a: i128,b: i128) -> i128 {
    let concatenation = format!("{}{}", a,b);
    return concatenation.parse::<i128>().unwrap_or(0);
}
// operations variable that is integral to our for loop
const operations: [fn(i128, i128) -> i128; 3] = [multiply,add,concatenate];

fn check_numbers(numbers: Vec<i128>, numberofoperations: usize) -> i128{
    // have to have answertotal = 0 here. sometimes it'll return nothing, if undefined it wont work.
    let mut answertotal = 0;
    let combinations = numberofoperations.pow((numbers.len() - 1) as u32);
    'outer: for i in 0..combinations{
        let mut runningtotal = numbers[1];
        let mut choosefunction = i;
        'inner: for j in 2..numbers.len() {
            let current = operations[(choosefunction % numberofoperations) as usize];
            runningtotal = current(runningtotal,numbers[j as usize]);
            // if we go over the total, check the next set of combinations. we can't decrease.
            if runningtotal > numbers[0]{
                break 'inner;
            }
            choosefunction /= numberofoperations;
            }
            // if we get a right answer once, extract the answer and go to the next row. we don't need it more than once.
            if runningtotal == numbers[0] {
                answertotal = numbers[0];
                break 'outer;
        }
    }
    //println!("{:?}",answertotal);
    answertotal
}
