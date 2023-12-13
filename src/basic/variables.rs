pub fn variables() {
    // integer
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("{}{}{}", a, b, c);

    // float
    let float1 = 5.5;
    let float2:f32 = 55.21;

    // boolean
    let bool1=false;
    let bool2 :bool= true;

    // character 4bytes soportan unicode
    let char1 = 'a'; 

    // constants
    const PI: f64 = 3.141592653589793;
    println!("PI : {}", PI);

    // shadowing elimina la anterior variable que es igual
    let value = 100;
    let value = 200;
    println!("value : {}", value);

}
