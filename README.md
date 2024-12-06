# Multiple Mutable Borrows in Rust

This repository demonstrates a common error in Rust: attempting to create multiple mutable references to the same variable.  This violates Rust's borrowing rules and will result in a compile-time error.  The `bug.rs` file contains the erroneous code, while `bugSolution.rs` shows a corrected version.