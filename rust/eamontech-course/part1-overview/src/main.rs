fn main() {
    let f: f64 = 5.0;
    let g: f32 = 4.0;
    let sum: i32 = 2 + 10;
    let subtract: i32 = 22 - 25;
    let multiplication: i32 = 5 * 3;
    let division: f64 = 192.2 / 11.6;
    let remainder: i32 = 45 % 12;
    println!("f = {}", f);
    println!("g = {}", g);
    println!("sum = {}", sum);
    println!("subtract = {}", subtract);
    println!("miltiplication = {}", multiplication);
    println!("division = {}", division);
    println!("remainder = {}", remainder);
    let tup = ("hello world", 1000_232_121);
    println!("string = {:?}, integer = {:?}", tup.0, tup.1);
    let (_string, _integer) = tup;
    println!("string = {:?}, integer = {:?}", _string, _integer);
    let number = [12, 45, 432];
    println!(
        "number 1 = {}, number 2 = {}, number 3 = {}",
        number[0], number[1], number[2]
    );
    let hashing = [0; 24];
    println!("hashing = {:?}", hashing);
}
