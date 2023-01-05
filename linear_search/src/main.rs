use std::io;

fn main() {
    let target = ask_target_value();
    let arr = ask_array_to_search();

    search_for_the_target(target, arr);
}

fn ask_target_value() -> i32 {
    println!("Please input value to search greater than 0");
    let mut target = String::new();

    io::stdin()
        .read_line(&mut target)
        .expect("failed to read from stdin");

    let trimmed = target.trim();

    match trimmed.parse::<i32>() {
        Ok(i) => return i,
        Err(..) => std::process::exit(1),
    }
}

fn ask_array_to_search() -> [i32; 5] {
    return [22, 10, 9, 7, 3];
}

fn search_for_the_target(target: i32, arr: [i32; 5]) {
    for n in arr {
        if n == target {
            println!("Value has been found");
            return;
        }
    }

    println!("Value does not exist in array");
}
