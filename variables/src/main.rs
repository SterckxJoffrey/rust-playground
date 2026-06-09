fn main() {
    let mut x = 5;
    println!("La valeur de x est : {}", x);
    x = 6;
    println!("La nouvelle valeur de x est : {}", x);

    let y = 5;
    let _y = y + 1;
    let y = 8;
    println!("La valeur de y est : {}", y);

    let z = cinq();
    println!("La valeur de x est : {}", z);
}

fn cinq() -> i32 {
    5 // valeur de retour donc pas de ; dedans
}
