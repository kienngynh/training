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
  - \[dev-dependencies\]: chứa các dependencies; cho việc ví dụ, test,
    và benchmark
  - \[build-dependencies\]: phần này có thể chỉ định các dependencies
    của cargo khác để sử dụng trong kịch bản của dự án
  - \[target\]: phần này được sử dụng để biên dịch chéo mã cho những mục
    tiêu khác
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
