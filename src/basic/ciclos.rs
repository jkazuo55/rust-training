pub fn ciclos() {
    // loop
    let mut count = 0;
    loop {
        println!("loop");
        count += 1;
        if count > 10 {
            break;
        }
    }

    // for
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("{}", number)
    }
    for number in 1..23 {
        println!("{}", number)
    }

    // while
    let mut cont2 = 0;
    while cont2 <= 10 {
        println!("{}", cont2);
        cont2 += 1;
    }
}
