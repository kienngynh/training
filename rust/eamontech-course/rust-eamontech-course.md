# Part 1: Overview

1.  Variable

<!-- end list -->

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

2.  Shadowing

<!-- end list -->

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

3.  Data types

<!-- end list -->

  - Scalar data
    
      - Integer
        
        | Length  | Signed | Unsigned |
        | :-----: | :----: | :------: |
        |  8-Bit  |   i8   |    u8    |
        | 16-Bit  |  i16   |   u16    |
        | 32-Bit  |  i32   |   u32    |
        | 64-Bit  |  i64   |   u64    |
        | 128-Bit |  i128  |   u128   |
        |  arch   | isize  |  usize   |
        

        ``` rust
        let a: i32 = 132_231; //Decimal
        let b: i32 = 0xda; //Hex
        let c: i32 = 0o12; //Octal
        let d: i32 = 0b1110101; //Binary
        let e: u8 = b'C'; //Byte (u8 only)
        println!("a = {}", a);
        println!("b = {}", b);
        println!("c = {}", c);
        println!("d = {}", d);
        println!("e = {}", e);
        ```
        
        ``` stdout
        a = 132231
        b = 218
        c = 10
        d = 117
        e = 67
        ```
    
      - Float
        
        <!-- end list -->
        
        ``` rust
        let f: f64 = 5.0;
        let g: f32 = 4.0;
        let sum: i32 = 2 + 10;
        let subtract: i32 = 22 - 25;
        let multiplication: i32 = 5 * 3;
        let division: f64 = 1997.2 / 11.6;
        let remainder: i32 = 45 % 12;
        println!("f = {}", f);
        println!("g = {}", g);
        println!("sum = {}", sum);
        println!("subtract = {}", subtract);
        println!("miltiplication = {}", multiplication);
        println!("division = {}", division);
        println!("remainder = {}", remainder);
        ```
        
        ``` stdout
        f = 5
        g = 4
        sum = 12
        subtract = -3
        miltiplication = 15
        division = 172.17241379310346
        remainder = 9
        ```
    
      - Boolean
        
        ``` rust
        let t = true;
        let f = false;
        ```
        
          - Character
        
        <!-- end list -->
        
        ``` rust
        let c = 'w';
        ```

  - Compound data
