use std::io;

fn calculate(opration: &str) {
    let my_opration: Vec<&str> = opration.split(" ").collect();
    let mut result: i64 = my_opration[0].parse::<i64>().unwrap();
    for i in 1..my_opration.len() {
        if i % 2 == 1 {
            match my_opration[i] {
                "+" => result += my_opration[i+1].parse::<i64>().unwrap(),
                "-" => result -= my_opration[i+1].parse::<i64>().unwrap(),
                "*" => result *= my_opration[i+1].parse::<i64>().unwrap(),
                "/" => result /= my_opration[i+1].parse::<i64>().unwrap(),
                _ => println!("someting else"),
            }
        }
    }
    println!("{:?}", result);
}

fn main() {
    let mut opration = String::new();
    println!("welcome to the rust_caluclator \n please enter your opration!");
    println!("use opration syntex eg (2 + 2, 23 * 5)");
    // reading line for user
    io::stdin().read_line(&mut opration).expect("failed to read the line");

    calculate(&opration.trim().to_string());
}
