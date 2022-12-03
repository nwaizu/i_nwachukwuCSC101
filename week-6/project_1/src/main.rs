use std::io;

fn area(base1:i32, base2:i32, height:i32, diagonal1:i32, diagonal2:i32) {
    let Area = (height / 2) * (base1 + base2);

    println!("Area Trapezium = {}", Area);
}

fn main() {
    let mut w = String::new();
    println!("
        WHAT WOULD YOU LIKE TO CALCULATE.
        1.Area of a Trapezium
        2.Area of a Rhombus
        3.Area of a Cube 
        4.Volume of a Cylinder");
    io::stdin()
    .read_line(&mut w).expect("Failed to read input");
    let w:i32 = w.trim().parse().expect("Invalid input"); 

    if w == 1 { println!(
         fn main() { 

        let mut base1 = String::new();
    println!("Enter input for parameter base1:");
    io::stdin().read_line(&mut base1).expect("Failed to read input");
    let a:i32 = base1.trim().parse().expect("Invalid input");

    let mut base2 = String::new();
    println!("Enter input parameter base2:");
    io::stdin().read_line(&mut base2).expect("Failed to read input");
    let b:i32 = base2.trim().parse().expect("Invalid input");

    let mut height = String::new();
    println!("Enter input parameter height:");
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let c:i32 = height.trim().parse().expect("Invalid input");

    let Area = (c * 1/2) * (a + b);

    println!("Area Trapezium = {}", Area);
     );
}
}
}  