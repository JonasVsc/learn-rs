fn main() {

    // let a = String::from("Jonas");
    // let b = String::from("Vulkan");

    // println!("a: {a}");
    // println!("b: {b}");

    // println!("swap!");
    // let (a, b) = swap(a, b);

    // println!("a: {a}");
    // println!("b: {b}");


    

    // let mut a = String::from("50 centavos");
    // let mut b = String::from("2 reais");

    // println!("a: {a}");
    // println!("b: {b}");

    // swap(&mut a, &mut b);

    // println!("a: {a}");
    // println!("b: {b}");


    /*
    write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
    If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
    */

    let text = "Hello, world!";
    let first_word: String = text[..5].to_string();
    println!("the first word of \"{text}\" is \"{first_word}\"");





}

// fn swap(a: String, b: String) -> (String, String) {
//     (b, a)    
// }

// fn swap(a: &mut String, b: &mut String) {
//     let temp = a.clone();
//     *a = b.clone();
//     *b = temp;
// }


