use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut worker = 0;
    let mut array: Vec<i32> = Vec::from([0]);
    for line in contents.lines() {
        match line {
            "" => {
                worker += 1;
                array.insert(worker, 0);
            },
            line =>
                array[worker] = array[worker] + line.parse::<i32>().unwrap(),
        }
    }
    let mut result = 0;
    let mut value = 0;
    for i in 0..array.len() {
        if array[i] > value {
            result = i;
            value = array[i];
        }
    }
    array.sort();
    let first = array[worker];
    let second = array[worker - 1];
    let third = array[worker - 2];
    let total = first + second + third;

    println!("{first}");
    println!("{second}");
    println!("{third}");
    println!("total: {total}")
}
