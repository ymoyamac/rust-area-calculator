use std::io;
use std::num::ParseIntError;

fn main() {
    println!("Area calculator");
    loop {
        println!("\
            Menu
            1.- Circle
            2.- Rectangle
            3.- Square
        ");
        println!("Please select a number");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Oops... Something went wrong.\n Error: ");
        let input = input.trim().parse::<i32>();

        match calc_area(input) {
            Ok(area) => {
                println!("Area: {}", area);
                break;
            },
            Err(error) => println!("Error: {}", error)
        }
    }

}

fn calc_area(choice: Result<i32, ParseIntError>) -> Result<f32, String> {
    match choice {
        Ok(1) => {
            println!("Circle area calculator");
            let radio = read_input();
            Ok(std::f32::consts::PI * radio * radio)
        },
        Ok(2) => {
            println!("Rectangle area calculator");
            let base = read_input();
            let height = read_input();
            Ok(base * height)
        },
        Ok(3) => {
            println!("Square area calculator");
            let side = read_input();
            Ok(side * side)
        },
        _ => Err("Invalid choice".to_string())
    }
}

fn read_input() -> f32 {
    println!("Please type a value:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let input = input.trim().parse::<f32>().expect("Failed to parse");
    input
}
