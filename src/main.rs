mod dynamic_dfa;
// use std::io;
use std::fs::read_to_string;
//use std::fs::File;

// fn read_user_input() -> String {
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");
//     input.trim().to_string()
// }
fn main() {
    
    println!("\nEnter a string to validate:");
    let input: Vec<String> = read_lines("./src/text.txt");
        if dynamic_dfa::validate(&input) == Ok(()) {
            print!("\nValidation Result:");
            println!(" Valid equation");
        } else {
            print!("\nValidation Result:");
            println!(" Invalid equation");
        }
        println!("\nEnter a string to validate:");
}




fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

//falta agregar al autómata ingresar numeros negativos, con punto decimal ,caracteres y id's con numeros
//funciones de gramatica
//asignacion de valores a variables
//creacion de tabla de tokens con su tipo y valor


