fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modifying x through y
    }
    {
        let z = &mut x; // z is a mutable reference to x
        *z = 15; // Modifying x through z
    }

    println!("x = {}", x); // This will print 15
}

// Note that we've now addressed this through scoping. Each mutable borrow is within its own block, so they don't overlap.