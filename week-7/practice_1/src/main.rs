use std::io;

fn main() {
    println!("\nStudentCouncil_VoteX function");

println!("\nAre you the class rep.
    Input 1 for YES
          2 for NO");
let mut classrep = String::new();
io::stdin()
.read_line(&mut classrep)
.expect("Failed to read input");
let c:f32 = classrep.trim().parse().expect("Failed to read input");

println!("What level are you in");
let mut level = String::new();
io::stdin()
.read_line(&mut level)
.expect("Failed to read input");
let l:f32 = level.trim().parse().expect("Failed to read input");

println!("\nWhat is your CGPA");
let mut cgpa = String::new();
io::stdin()
.read_line(&mut cgpa)
.expect("Failed to read input");
let g:f32 = cgpa.trim().parse().expect("Failed to read input");

if c == 2.0 && c == 1.0 && l == 100.0 && g <= 4.0 { println!("Sorry,you are not eligible to vote");}  

if c == 1.0 && l > 100.0 && g > 4.0 { println!("You can vote"); 

// input name
println!("Please fill in your name.");
let mut name = String::new();
io::stdin()
.read_line(&mut name)
.expect("Failed to read input");

//input email
println!("Please put your email.");
let mut email = String::new();
io::stdin()
.read_line(&mut email)
.expect("Failed to read input");

//input department
println!("Please input your department.");
let mut department = String::new();
io::stdin()
.read_line(&mut department)
.expect("Failed to read input");

//input State of orgin
println!("What is your state of origin");
let mut sog = String::new();
io::stdin()
.read_line(&mut sog)
.expect("Failed to read input");
println!("Thank you");


}  


}
