fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x

    *y = 10; // Modifying x through y
    *z = 15; // Modifying x through z

    println!("x = {}", x); // This will print 15
}