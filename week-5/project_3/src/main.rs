use std::io;

fn main() {
    println!("
        TODAY'S MENU 
        FOOD                                  PRICE
        P = Poundo Yam/Edinkaiko Soup        -N3,200
        F = Fried Rice & Chicken             -N3,000
        A = Amala & Ewudu Soup               -N2,500
        E = Eba & Egusi Soup                 -N2,000
        W = White Rice & Stew                -N2,500");

    let mut food = String::new();
    let mut quantity = String::new();
    let mut price = String::new();

    println!("What would you like to eat?
            Input 1 for Poundo Yam/Edinkaiko Soup,
            Input 2 for FRied Rice & Chicken,
            Input 3 for Amala & Ewudu Soup,
            Input 4 for Eba & Egusi Soup,
            Input 5 for White Rice & Stew:");
        io::stdin()
        .read_line(&mut food)
        .expect("Not a valid string");
    let a:f32 = food.trim().parse().expect("Not a valid response");

    let P:f32 = 1.0;
    let F:f32 = 2.0;
    let A:f32 = 3.0;
    let E:f32 = 4.0;
    let W:f32 = 5.0;

    println!("\nHow much of it do you want");
        io::stdin().read_line(&mut quantity).expect("Not a valid string");
    let b:f32 = quantity.trim().parse().expect("Not a valid response");
    
     let p:f32 = price.trim().parse().expect("Not a valid response");

      if (a > 0.0) && (b >= 0.0) { 
        let p = b * 3200.00;
        println!("Total charges is N{}", p);

    if (a == 2.0) && (b >= 0.0) {
        let p = b * 3000.00;
        println!("Total charges is N{}", p);}

    if (a == 3.0) && (b >= 0.0) {
        let p = b * 2500.00;
        println!("Total charges is N{}", p);}

    if (a == 4.0) && (b >= 0.0) {
        let p = b * 2000.00;
        println!("Total charges is N{}", p);}

    if (a == 5.0) && (b >= 0.0) {
        let p = b * 2500.00;
        println!("Total cahrges is N{}", p);}

    let mut Total:f32 = p * 1.0;
    if Total > 10000.00{ Total = p * (5.0 / 100.00);
        println!("Total charges is N{}", p);}

        println!("Thank you for your purchase");
}
}