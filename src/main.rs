use std::io::{self, Write};

fn main() {
    println!("Welcome to the Dimension Calculator!");

    loop {
        // main dimension selection menu
        println!("\nSelect a dimension:");
        println!("1. 2D Shapes");
        println!("2. 3D Shapes");
        println!("0. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => show_2d_menu(),
            "2" => show_3d_menu(),
            "0" => {
                println!("Thank you for using the Dimension Calculator!");
                break;
            }
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

fn show_2d_menu() {
    println!("\n2D Shapes Menu:");
    println!("1. Circle");
    println!("2. Triangle");
    println!("3. Rectangle");
    println!("0. Back to main menu");

    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim();

    match choice {
        "1" => calculate_circle(),
        "2" => calculate_triangle(),
        "3" => calculate_rectangle(),
        "0" => return,
        _ => println!("Invalid choice! Please try again."),
    }
}

fn show_3d_menu() {
    println!("\n3D Shapes Menu:");
    println!("1. Sphere");
    println!("2. Pyramid");
    println!("3. Rectangular Prism");
    println!("0. Back to main menu");

    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim();

    match choice {
        "1" => calculate_sphere(),
        "2" => calculate_pyramid(),
        "3" => calculate_rectangular_prism(),
        "0" => return,
        _ => println!("Invalid choice! Please try again."),
    }
}

// Placeholder functions for calculations
fn calculate_circle() {
    println!("Circle calculation selected (not yet implemented)");
    // We'll implement this next
}

fn calculate_triangle() {
    println!("Triangle calculation selected (not yet implemented)");
}

fn calculate_rectangle() {
    println!("Rectangle calculation selected (not yet implemented)");
}

fn calculate_sphere() {
    println!("Sphere calculation selected (not yet implemented)");
}

fn calculate_pyramid() {
    println!("Pyramid calculation selected (not yet implemented)");
}

fn calculate_rectangular_prism() {
    println!("Rectangular Prism calculation selected (not yet implemented)");
}