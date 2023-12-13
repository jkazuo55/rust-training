// fn palabra reservada
// nombre de funcion con barra baja
// las funciones pueden ir antes o despues

pub fn try_functions() {
    // high order function
    let value = 5;
    let res = high_order_function(double, value);
    println!("high_order_function return : {:?}", res);

    // return value
    println!("return function : {:?}", return_function());

    // void function
    void_function();

    // funtion parameter option Some()
    let optional_value = Some(100);
    println!(
        "option_parameter_function {:?}",
        option_parameter_function(10, optional_value)
    );
    // funtion parameter option None
    let optional_value = None;
    println!(
        "option_parameter_function {:?}",
        option_parameter_function(10, optional_value)
    );

    // parameter with reference function
    let name = "julian";
    println!(
        "parameter_with_reference_function {:?}",
        reference_parameter_function(name)
    )
}

// return a value
fn return_function() -> i32 {
    return 10;
}

// void function
fn void_function() -> () {
    println!("void function")
}

// high order function
// functional programming
// accept a function parameter
// return other functions
fn high_order_function(function: fn(i32) -> i32, value: i32) -> i32 {
    return function(value);
}

// function parameter options
fn option_parameter_function(a: i32, b: Option<i32>) -> i32 {
    match b {
        Some(value) => {
            return value;
        }
        None => return 0,
    }
}

// function parameter whit references
fn reference_parameter_function(name: &str) {
    
    name;
}

// Aux function
fn double(number: i32) -> i32 {
    return number * 2;
}
