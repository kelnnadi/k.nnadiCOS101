use std::io;

fn read_number(prompt: &str) -> f64 {
    let mut input = String::new();

    println!("{}",prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().parse::<f64>().expect("Please enter a valid number")

}

fn trapezium_area() -> f64 {
    let base1 = read_number("Enter the first parallel base:");
    let base2 = read_number("Enter the second parallel base:");
    let height = read_number("Enter the height:");
    height / 2.0 * (base1 + base2)
}

fn rhombus_area() -> f64 {
    let diagonal1 = read_number("Enter the first diagonal1:");
    let diagonal2 = read_number("Enter the second diagonal:");

    0.5 * diagonal1 * diagonal2
}

fn parallelogram_area() -> f64 {
    let base = read_number("Enter the base:");
    let altitude = read_number("Enter the altitude:");

    base * altitude

}

fn cube_surface_area() -> f64 {
    let side = read_number("Enter the side length:");

    6.0 * side * side
}

fn cylinder_volume() -> f64 {
    let radius = read_number("Enter the radius:");
    let height = read_number("Enter the height:");

    std::f64::consts::PI * radius * radius * height 
}

 fn main() {
    println!("===SHAPE CALCULATOR===");
    println!("1. Trapezium - Area");
    println!("2. Rhombus - Area");
    println!("3. Parallelogram- Area");
    println!("4. cube - Surface Area");
    println!("5. cylinder - Volume");

    let choice = read_number("Choose a shape (1-5):");
    let result = match choice as u32 {
        1 => trapezium_area(),
        2 => rhombus_area(),
        3 => parallelogram_area(),
        4 => cube_surface_area(),
        5=> cylinder_volume(),
        _ => {
            println!("Invalid choice. Please choose from 1 to 5.");
            return;
        }
    };

    println!("The answer is: {}",result);

}

