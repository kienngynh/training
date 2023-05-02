# Chương 1

## Cài đặt Rust

``` bash
 curl --proto ‘=https’ --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

  - **rustc**: là trình biên dịch của Rust. Thông thường trình biên dịch
    *rustc* sẽ được gọi thông qua *cargo* mà không được gọi trực tiếp.
  - **cargo**: là công cụ xây dựng và quản lý gói của Rust. Có thể sử
    dụng *cargo* để tạo một dự án mới, xây dựng và chạy chương trình.
    và quản lý bất kỳ thư viện mở rộng nào mà chương trình cần.
  - **rustdoc**: là công cụ tài liệu của Rust, có thể sử dụng *cargo* để
    chạy *rustdoc*.

## Cập nhật và gỡ cài đặt

``` bash
rustup update
rustup self uninstall
```

## Làm việc với Cargo

  - Cargo.toml: file cấu hình dự án
      - \[dependencies\]: chứa các gói thư viện hoặc file nhị phân
      - \[dev-dependencies\]: chứa các dependencies; cho việc ví dụ,
        test, và benchmark
      - \[build-dependencies\]: phần này có thể chỉ định các
        dependencies của cargo khác để sử dụng trong kịch bản của dự án
      - \[target\]: phần này được sử dụng để biên dịch chéo mã cho những
        mục tiêu khác
  - Một số lệnh cargo:

<!-- end list -->

``` bash
cargo new # tạo thư mục dự án mới
cargo init # khởi tạo dự án trong thư mục đã tạo
cargo check # kiểm tra code và biên dịch nhưng không tạo file thực thi
cargo run # build và run chương trình
cargo build --dev # build chương trình mặc định
cargo build --release # cho phép xuất bản hồ sơ
cargo build --test # build chương trình thử
cargo build --bench # tạo chương trình benchmark
```

# Chương 2

## Variables

``` rust
let x = 17;
println!("x = {}", x);
```

``` stdout
x = 17
```

## Mutability

  - Trong ngôn ngữ Rust, khi khai báo một biến thì biến đó mặc định được
    coi là bất biến, giá trị của biến đó không thể thay đổi.
  - Để cho biết biến đó là biến có thể thay đổi giá trị, cần thêm mut ở
    trước tên biến.

<!-- end list -->

``` rust
let mut x = 26;
println!("x = {}", x);
x = 11;
println!("x = {}", x);
```

``` stdout
x = 26
x = 11
```

## Kiểu dữ liệu

  - Do là một ngôn ngữ lập trình tĩnh nên tại thời điểm biên dịch, Rust
    phải biết kiểu dữ liệu của biến
  - Có hai dạng dữ liệu chính trong Rust: Scalar và Compound.

<!-- end list -->

``` rust
let y:u32 = "10".parse().unwrap();
println!("{}",y);
```

``` stdout
10
```

### Scalar data types

1.  Integer

<!-- end list -->

  - Số nguyên là đại diện cho các số không có phân số. Có nhiều cách
    khác nhau để đại diện cho số nguyên trong Rust, tuỳ thuộc vào số
    lượng bit có dấu hoặc không có dấu

| Số lượng bit          | Có dấu       | Không dấu |
| :-------------------- | :----------- | :-------- |
| 8-bit                 | i8           | u8        |
| 16-bit                | i16          | u16       |
| 32-bit                | i32(default) | u32       |
| 64-bit                | i64          | u64       |
| 128-bit               | i128         | u128      |
| architecture-specific | isize        | usize     |

2.  Floating-point

<!-- end list -->

  - Trong Rust có cung cấp kiểu dữ liệu f32 và f64(mặc định) để chứa số
    thập phân

<!-- end list -->

``` rust
let x: f64 = 3.0;
let y = 3.0f64;
let z = 3.0 as f64;
println!("{} {} {}",x,y,z);
```

``` stdout
3 3 3
```

3.  Boolean

<!-- end list -->

  - Kiểu dữ liệu boolean chứa 2 giá trị: true hoặc false và có một byte
    dung lượng, thường được sự dụng trong mệnh đề điều kiện…

<!-- end list -->

``` rust
let t = true;
let f:bool = false;
println!("{} {}",t,f);
```

``` stdout
true false
```

4.  Characters
