pub fn matchs() {
    let number = 3;
    match number {
        1 => println!("number 1"),
        2 => println!("number 2"),
        3 => println!("number 3"),
        5 | 6 | 7 | 8 => println!("number se encuentra entre 5 y 8"),
        10..=100 => println!("number mayor a 10"),
        _ => println!("{}", number), //default
    }
}
