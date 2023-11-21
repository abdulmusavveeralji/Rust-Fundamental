fn add(num_one: i32, num_two: i32) -> i32 {
    return num_one + num_two;
}

fn main() {
    let mut _my_name: &str = "Musavveer";
    _my_name = "Aljis";
    let mut sum = add(230, 64);
    let mut free_shipping = false;

    if sum > 50 {
        print!("You qualify for free shipping!");
        free_shipping = true;
    } else if sum > 20 {
        print!("If you add more items, You can qualify for free shipping!");
    } else {
        print!("No Free shipping!");
    }

    match free_shipping {
        true => sum = sum + 0,
        false => sum = sum + 5,
    };

    println!("Total: {:?} {0}", sum);
    println!("Hello, world!");
}
