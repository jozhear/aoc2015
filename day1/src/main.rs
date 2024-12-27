use std::fs;

fn main() {
    let data = fs::read_to_string("day1.txt").expect("Couldn't open it");
    let newdata: Vec<String> = data
        .chars().map(|ch| {
            ch.to_string()
        })
        .collect();
    let mut answersum: i32 = 0;
    let mut position: i32 = 0;
    let mut positionvec: Vec<i32> = Vec::new();
    for floor in newdata {
        if floor == "(" {
            answersum +=1;
        } else {
            answersum -=1;
        }
        position +=1;
        if answersum == -1 {
            positionvec.push(position);
        }
    }
    println!("{:?}",answersum);
    println!("{:?}",positionvec[0]);
}
