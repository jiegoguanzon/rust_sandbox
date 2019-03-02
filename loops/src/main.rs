fn main() {
    
    loop {
        println!("Again!");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Counter value: {:?}", result);

    let mut i = 3;
    while i > 0 {
        println!("{}!", i);
        i -= 1;
    }
    println!("Liftoff!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is {}", element);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }
    println!("Liftoff!");

}
