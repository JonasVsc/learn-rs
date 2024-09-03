// use std::io;

fn main() {

    // somatorio com loop
    // let mut number = String::new();

    // read user input
    // io::stdin()
    //     .read_line(&mut number)
    //     .expect("Failed reading user input!");

    // let mut number: usize = number.trim().parse().expect("Not a Number!");

    // let mut sum = 0;

    // let result = loop {
    //     sum += number;
    //     number -= 1;

    //     if number == 0 {
    //         break sum;
    //     }
    // };
    // // println!("The sum of all numbers 0..{number} is {result}");


    // let result = number * (number + 1) / 2;
    // println!("The sum of all numbers 0..{number} is {result}");



    // let mut number = 10;

    // while number != 0 {
    //     println!("{number}");
    //     number -= 1;
    // }


    // let my_array = [1, 2, 3, 4, 5];

    // for i in my_array {
    //     println!("value is: {i}");
    // }


    // Convert temperatures between Fahrenheit and Celsius.

    // use std::io;

    // let mut fahrenheit = String::new();

    // println!("Insert °F: ");
    // io::stdin()
    //     .read_line(&mut fahrenheit)
    //     .expect("Failed read line");

    // // converting string to int
    // let fahrenheit: i16 = fahrenheit.trim().parse().expect("Not a number!");

    // println!("Converting to celsius...");
    // // °C = (°F - 32) × 5/9
    // let celsius = (fahrenheit - 32) * 5/9;
    // println!("{fahrenheit} °F in celius is: {celsius}°C !");


    // Generate the nth Fibonacci number.

    /* fib formula
        
        if 5

        1, 1, 2, 3, 5, 8
        
        1) special case = 1
        2) 0 + 1 = 1 
        3) 1 + 1 = 2
        4) 1 + 2 = 3
        5) 2 + 3 = 5
        6) 3 + 5 = 8
           i   j   k
           
        algorithm
        input: num value
        output: fibonnaci sequence

    */
    use std::io;

    let mut num = String::new();

    println!("Insert a number");
    io::stdin()
        .read_line(&mut num)
        .expect("Failed read line");

    let mut num: isize = num.trim().parse().expect("Not a Number");

    let mut i = 0;
    let mut j = 1;

    println!("1 + 0 = 1"); // special case
    while num > 1 {
        let k = i + j;
        println!("{i} + {j} = {k}");
        i = j;
        j = k;

        num -= 1;
    }
}
