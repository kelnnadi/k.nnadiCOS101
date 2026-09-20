use std::io;

fn main() 
{
    let mut experience = String::new();
    let mut age_input = String::new();


    println!("Are you experienced? (yes/no): ");
    io::stdin().read_line(&mut experience).expect("Failed to read input");


    println!("Enter your age: ");
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: i32 = age_input.trim().parse().expect("please enter a valid age");

    let experience = experience.trim();

    let mut incentive : i32 = 0 ;

    if experience == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age <= 39 {
            incentive = 1_480_000;
        } else if age <28 {
            incentive = 1_300_000;
        }
    }


    else if experience == "no" {
        incentive = 100_000;
    } else { 
        println!("please enter yes or no.");
    }


    println!("Your incentive is Naira{}",incentive);



}




