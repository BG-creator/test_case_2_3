
use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {

    println!("############ Welcome to Administrator interface ###########");

    loop{
        println!("1.Add User ");
        println!("2.Add Organization");
        println!("3.User from the department");
        println!("4.ALl user");
        println!("5.Exit");

        let choice = get_input();

        match choice.as_str(){
            "1" => add_user(),
            "2" => add_organization(),
            "3" => add_to_department(),
            "4" => all_user(),
            "5" => {
                println!("EXIT........................");
                break;
            }
            _ => println!("Invalid choice"),

        }


    }

}


fn add_user(){
    println!("Add User");
}
fn add_organization(){
    println!("Add Organization");
}
fn add_to_department(){
    println!("Add To Department");
}
fn all_user(){
    println!("ALL Users");
}
fn get_input()-> String{
    let mut user_input = String::new();
    print!("Please enter you choice:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input.trim().to_string()
}