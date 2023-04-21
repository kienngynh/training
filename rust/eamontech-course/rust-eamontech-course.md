# Part 1: Overview

## Variable

  - The default variable in Rust is immutable
  - Syntax declare a const is: const + LETTER\_UPCASE: datatype = value

<!-- end list -->

``` rust
let mut x = 10;
println!("x ={}", x);
x = 20; 
println!("x = {}", x);
const HANG_SO: u64 = 100_000_000_000;
println!("HANG SO = {}", HANG_SO);
```

``` stdout
x = 10
x = 20
HANG SO = 100000000000
```

## Shadowing

  - While declaring a variable with the name is exist, the old variable
    doesn’t disappear. It is shadowed by the new value.

<!-- end list -->

``` rust
let outer = 10; 
{ let inner = 200; 
  println!("inner = {}", inner); 
  let outer = 300; 
  println!("outer = {}", outer); 
}
println!("outer = {}", outer);
```

``` stdout
inner = 200
outer = 300
outer = 10
```

## Data types

### Scalar data

| Length  | Signed | Unsigned |
| :-----: | :----: | :------: |
|  8-Bit  |   i8   |    u8    |
| 16-Bit  |  i16   |   u16    |
| 32-Bit  |  i32   |   u32    |
| 64-Bit  |  i64   |   u64    |
| 128-Bit |  i128  |   u128   |
|  arch   | isize  |  usize   |

``` rust
let a: i32 = 111_111; println!("a = {}", a);
```

``` stdout
a = 111111
```

### Compound data
