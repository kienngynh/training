Part 1: Overview
================

-   Variable
    -   The default variable in Rust is immutable
    -   Syntax declare a const is: const + LETTER\_UPCASE:datatype =
        value

``` {.rust .numberLines startFrom="1"}
let mut x = 10;
println!("x = {}", x);
x = 20;
println!("x = {}", x);
const HANG_SO: u64 = 100_000_000_000;
println!("HANG SO = {}", HANG_SO); 
```

``` {.stdout}
x = 10
x = 20
HANG SO = 100000000000
```
