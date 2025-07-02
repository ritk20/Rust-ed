fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let tup = (500, 6.4, 1);
    let (_, y, _) = tup;
    println!("The value of y is: {y}");
    println!("The value of x is: {0}", tup.0);

    let _a: [i32; 5] = [1, 2, 3, 4, 5]; //array of type i32 with 5 elements

    let b = [3; 5]; //initializes an arr of 5 elements, each with val 3
    println!("b[0] = {}", b[0]);

    let c = plus_one(5);
    println!("c = {c}");

    // let d = plus_one_not_working(5);
    // println!("d = {d}"); // This will not compile because the function does not return a value
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// fn plus_one_not_working(x: i32) -> i32 {
//     x + 1;
// }
