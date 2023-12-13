pub fn semaphore() {
    let color = "green";
    if color == "green" {
        println!("can continue")
    } else if color == "yellow" {
        println!("stop partial")
    } else if color == "red" {
        println!("stop" )
    }
    else {
        println!("the color is not valid")
    }
}