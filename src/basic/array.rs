pub fn arrays() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{:?}", numbers);

    // only numbers 
    let numbers: [i32; 6] = [2, 3, 4, 5, 6, 7];
    println!("{:?}", numbers);

    // repetir 10 veces el 1
    let numbers= [1; 10];
    println!("{:?}", numbers);

    
}
