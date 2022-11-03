fn main() {
    
    let n1 = "Eletrical".to_string();
    let n2 = " Eletronic".to_string();
    let n3 = " Engineering".to_string();
    let n4 = n1 + &n2 + &n3; // n2 & n3 reference is

    // About Electrical/Eletronic
    println!("\nThe {} is informed by the aspiration to
        train eletrical/eletronic engineering professionals
        in the areas of design, building and maintenance of 
        eletrical control systems,", n4);

    let w1 = "Computer".to_string();
    let w2 = " Science".to_string();
    let w3 = w1 + &w2;  // w2 reference is passed
    println!();
    println!("{} is aimed at developing competent, creative,
        innovative, entrepreneural and ethically-minded persons,
        capable of creating value in the diverse fields of 
        Computer Science. ",w3);
    
}
