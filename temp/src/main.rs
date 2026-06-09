use std::io;

fn main() {
    loop {
        println!("------Menu-------");
        println!("1. Degrée to Fahrenheit");
        println!("2. Degrée to Celsius");
        println!("3. Choisissez une option");

        let mut choix = String::new();
        io::stdin()
            .read_line(&mut choix)
            .expect("Erreur de lecture");

        match choix.trim() {
            "1" => {
                convertir_degree_to_fahrenheit();
            }
            "2" => {
                convertir_degree_to_celsius();
            }
            "3" => {
                println!("Au revoir !");
                break;
            }
            _ => {
                println!("Option invalide")
            }
        }
    }
}

fn convertir_degree_to_fahrenheit() {
    println!("Entrez une valeur en degrés Celcius :");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");

    match input.trim().parse::<f64>() {
        Ok(c) => {
            let f = c * 1.8 + 32.0;
            println!("La valeur en degrés Fahrenheit est : {}", f);
        }
        Err(_) => {
            println!("Valeur invalide");
        }
    }
}

fn convertir_degree_to_celsius() {
    println! {"Entrez une valeur en degrés Celsius : "}
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");
    match input.trim().parse::<f64>() {
        Ok(c) => {
            let celsius = c * 9.0 / 5.0 + 32.0;
            println!("La valeur en degrés Celsius est : {}", celsius);
        }
        Err(_) => {
            println!("Valeur invalide");
        }
    }
}
