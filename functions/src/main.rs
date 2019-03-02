fn main() {

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {}.", y);
    another_function(3, 4.5);
    let x = add_one(y);
    println!("The value of x is {}.", x);

}

fn another_function(x: i32, y: f32) {

    println!("The value of x is {}.", x);
    println!("The value of y is {}.", y);

}

fn add_one(x: i32) -> i32 {
    x + 1
}