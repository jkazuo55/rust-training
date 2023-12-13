// un bloque un conjunto se sentencias agrupadas por llaves

pub fn block() {
    let message = "this is a message ";
    let sub_block = {
        let variable = "content sub block";
        variable
    };

    println!("{}", message);
    println!("{:?}", sub_block);
}

pub fn califications() {
    let calification: i8 = 10;
    // define message empty
    let mut message = String::new();

    if calification == 10 {
        // convierte la cadena literal en objeto string
        message = String::from("congratulations");
    } else {
        // convierte la cadena literal en objeto string
        message = String::from("reprobed")
    }
}
