use std::io;

fn main() {
    println!("Lets determine your annual incentive");
        let mut Experience = String::new();
        let mut Age = String::new();

    println!("Are you experienced? Input 1 for Yes and 0 for NO:");
        io::stdin()
        .read_line(&mut Experience)
        .expect("Not a valid string");
    let a:f32 = Experience.trim().parse().expect("Not a valid number");

    println!("How old are you:");
        io::stdin()
        .read_line(&mut Age)
        .expect("Not a valid string");
    let b:f32 = Age.trim().parse().expect("Not a valid number");

    if a == 1.0 && b >= 40.0 { println!("Your incentive is N1,560,000.00");}
    if a == 1.0 && b >= 30.0 && b < 40.0 { println!("Your incentive is N1,480,000.00");}
    if a == 1.0 && b < 28.0 { println!("Your incentive is N1,300,000.00");}
    if a == 0.0 { println!("Ypur incentive is N100,000.00");}
}
