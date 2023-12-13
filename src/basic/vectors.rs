// modificar su longitud 
// almacena un mismo tipo de dato

/*
* se usa cuando tengo los valores iniciales
*/
pub fn vector_with_macro() {
    let mut vector = vec![1, 2, 3];
    vector.push(4);
    vector.push(5);
    vector.insert(0, 50);
    vector.remove(vector.len()-1); 
    vector[1]=100;
    println!("{:?}", vector);
}  

/*
* se usa cuando no tengo los valores iniciales
*/
pub fn vector_with_structure() {
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);
    vector.push(5);
    vector.insert(0, 50);
    vector.remove(vector.len()-1); 
    vector[1]=100;
    println!("{:?}", vector);

}