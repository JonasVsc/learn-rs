fn main() {
    another_function(2312);
}

fn another_function(x: isize) {
    let x = five(x);
    let x = plus_one(x);
    println!("value of x is {x}");
}

fn five(x: isize) -> isize {
    x
}

fn plus_one(x: isize) -> isize {
    x + 1
}