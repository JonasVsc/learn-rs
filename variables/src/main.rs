use std::io;

fn main() {
    // access array variables

    let my_array: [isize; 5] = [1,2,3,4,5];

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");


    let index: usize = index
        .trim()
        .parse()
        .expect("Not a number!");


    let element = my_array[index];
    println!("Index at {index} is {element}");
}



