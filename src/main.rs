use std::env;

struct Fruit {
    name: String,
}

impl Fruit {
    fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args[1]);

    let x: i32 = args[1].parse().unwrap();
    let y: i32 = args[2].parse().unwrap();

    println!("{:?}", x + y)
}

#[test]
fn calc_test() {
    let x = 1;
    let y = 3;

    assert_eq!(x + y, 4)
}
