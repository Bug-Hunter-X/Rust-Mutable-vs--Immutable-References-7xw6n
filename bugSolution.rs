fn main() {
    let mut x = 5;
    let y = &mut x; // Mutable reference
    let z = &x;     // Immutable reference

    *y += 1; // Allowed: Modifying through a mutable reference
    println!("x (modified through mutable ref) = {}", x); // Output: x = 6

    println!("x (accessed via immutable ref) = {}", *z); // Allowed: Accessing through immutable reference
    // The following line will cause a compile-time error because we are trying to modify through an immutable reference
    //*z += 1;
} 