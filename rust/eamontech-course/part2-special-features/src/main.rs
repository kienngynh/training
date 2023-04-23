fn main() {
    fn _x() {
        let _z = "hello";
        let _w = 100;
        _y();
    }
    fn _y() {
        let mut _s = "world".to_owned();
        _s.push_str("!");
    }

    let mut _s1 = "hello".to_owned();
    let _s2 = &_s1;
    println!("{}", _s2);
}
