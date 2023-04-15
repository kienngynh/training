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

-   Shadowing
    -   While declaring a variable with the name is exist, the old
        variable doesn't disappear. It is shadowed by the new value.

``` {.rust .numberLines startFrom="7"}
let outer = 10;
{
  let inner = 200;
  println!("inner = {}", inner);
  let outer = 300;
  println!("outer = {}", outer);
}
  println!("outer = {}", outer);
```

``` {.stdout}
inner = 200
outer = 300
outer = 10
```

-   Data types

  Hello   World
  ------- -------
  a       b

-   Scalar data

-   Compound data
