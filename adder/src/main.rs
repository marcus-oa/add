use add_one;
use add_two;
use rand;

fn main() {
    let num = 10;
    println!(
        "Hello, world! \n {} plus one is {} \n {} plus two is {}",
        num,
        add_one::add_one(num),
        num,
        add_two::add_two(num)
    );
}
