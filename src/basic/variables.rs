pub fn variables() {
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("{}{}{}", a, b, c);

    // constants
    const PI: f64 = 3.141592653589793;
    println!("PI : {}", PI);

    // shadowing elimina la anterior variable que es igual
    let value = 100;
    let value = 200;
    println!("value : {}", value);

}
