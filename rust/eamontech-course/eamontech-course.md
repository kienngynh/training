Part 1: Overview
================

-   Variable
    -   Default variable in Rust is immutable
    -   Syntax declare a const is: const + LETTER\_UPCASE:<datatype> =
        value

<!-- -->

    let mut x = 10;
    rprintln!("x = {}", x);
    x = 20;
    rprintln!("x = {}", x);
    const HANG_SO: u64 = 100_000_000_000;
    rprintln!("HANG SO = {}", HANG_SO); 

    ## x = 10
    ## x = 20
    ## HANG SO = 100000000000
