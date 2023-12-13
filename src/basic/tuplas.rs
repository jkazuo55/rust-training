// alamcena diferentes tipos de datos

pub fn tuplas() {
    let tupla = (10, "hola", false, 10.6);
    println!("{:?}", tupla); 
    
    let tupla: (i32, bool, f64) = (10, false,5.5);
    println!("{:?}", tupla); 
    
    let first_element= tupla.0;
    println!("{:?}", first_element); 

}
 