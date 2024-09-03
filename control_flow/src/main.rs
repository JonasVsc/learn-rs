fn main() {
    let mut test = {
        let alguma_coisa: isize = "3".trim().parse().expect("Not a Number");
        alguma_coisa + 1
    };

    loop {
        test = {
            if test > 10 {
                println!("teste is greater than 10: {test}");
                test - 10
            } else {
                println!("teste is lower than 10: {test}");
                test + 99
            }
        };
        println!("teste value: {test}");
        if test < 10 { break };
    }

}
