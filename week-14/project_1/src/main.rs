use std::io::Read;
fn globacom_table () {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);}

fn staff_table () {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);}

fn customer_table () {
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);}

fn dataplan_table () {
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);}

fn department_table () {
    let mut file = std::fs::File::open("department_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);}

fn project_table () {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);}





use std::io;

fn main() {
    println!(
        "\nWelcome To The Globacom Database Schema Access Automated System.
        Please select an ID number.

        Administrator ( Access to the Globacom Database ) = 1
        Employee ( Access to the Staff table ONLY ) = 2
        Customer ( Access to the Customer table ONLY ) = 3
        Vendor ( Access to the Dataplan table ONLY ) = 4
        Project Manager ( Access to the Project table ONLY ) = 5 ");

let mut input = String::new();

io::stdin().read_line(&mut input).expect("Not a valid string");
let id:i32 = input.trim().parse().expect("Not a valid number");

if id == 1 {
    println!("Welcome to the Globacom Database Schema Access Automated System, as the ADMINISTRATOR you have access to the following,
     Below is the Globacom Database {:?}",globacom_table());}

if id == 2 {
     println!("Welcome to the Globacom Database Schema Access Automated System, as an EMPLOYEE you have access to the following,
     Below is the table for the staff {:?}",staff_table());}

if id == 3 {
     println!("Welcome to the Globacom Database Schema Access Automated System, as a CUSTOMER you have access to the following,
     Below is the table for customers {:?}",customer_table());}

if id == 4 {
     println!("Welcome to the Globacom Database Schema Access Automated System, as a VENDOR you have access to the following,
     Below is the Dataplan table {:?}",dataplan_table());}

if id == 5 {
     println!("Welcome to the Globacom Database Schema Access Automated System, as a Project Manager you have access to the following,
     Below is the Project table {:?}",project_table());}
}
