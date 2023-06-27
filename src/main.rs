mod dynamic_dfa;
use std::fs::read_to_string;

fn main() {
    
    println!("\nEnter a string to validate:");
    let input: Vec<String> = read_lines("./src/text.txt");
    let validation = dynamic_dfa::validate(&input);
        if validation.is_empty() {
            print!("\nValidation Result:");
            println!(" Valid equation");
        } else {
            print!("\nValidation Result:");
            println!(" Invalid equation");
            for error in validation {
                println!("{}", error);
            }

        }
}




fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

//funciones de gramatica
//asignacion de valores a variables
//creacion de tabla de tokens con su tipo y valor


