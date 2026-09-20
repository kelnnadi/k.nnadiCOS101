
use std::io;

fn  main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("Enter a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let a : f64 =input1.trim().parse().expect("Not a valid Number");


    println!("Enter b: ");
    io::stdin().read_line(&mut input2).expect("not a valid String");
    let b :f64 = input2.trim().parse().expect("Not a valid number");


    println!("Enter c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid String");
    let c :f64 = input3.trim().parse().expect("Not a valid number");


    let d = b*b - 4.0 * a * c ;

    if d > 0.0 {
        let x1 = -b + d.sqrt()/(2.0 * a );
        let x2 = -b - d.sqrt()/(2.0 * a );
            println!("two distinct real roots: ");
            println!("x1 = {}",x1);
            println!("x2 = {}",x2);
        }

    else if d == 0.0 {
        let x = -b / (2.0 * a);
        println!("exactly one real root");
        println!("x = {}",x)

    }

    else   {
        //no real roots
        println!("No real roots.");
    }





}
