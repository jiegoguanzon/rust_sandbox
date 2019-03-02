fn main() {

    // Mutable variables
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces is {}", spaces);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("Max points is {}", MAX_POINTS);

    // Scalar Data Types
    // Integer
    let i: u32 = 100_000;
    let j: i32 = 100_000;
    println!("i: {} j: {}", i, j);

    // Float
    let k: f64 = 3.2;
    println!("k: {}", k);

    // Boolean
    let state: bool = false;
    println!("State: {}", state);

    // Character
    let letter = '3';
    println!("Letter: {}", letter);

    // Compound Data Types
    // Tuple
    let tup: (i32, f32, bool) = (10, 3.3, true);
    let (u, v, w) = tup;
    let element = tup.0;
    println!("tup: {:?} u: {} v: {} w: {} element: {}", tup, u, v, w, element);


    // Array
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = a[0] + a[1];
    println!("a: {:?} b: {}", a, b);

}
