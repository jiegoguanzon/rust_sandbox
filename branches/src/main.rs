fn main() {
    
    let number = 3;

    if number < 5 {
        println!("Number is less than five.");
    } else if number > 5 {
        println!("Number is greater than five.");
    } else {
        println!("Number is equal to five.");
    }

    let condition = true;
    let x = if condition {
        5
    } else {
        6
    };
    
    println!("X is {}", x);

}
