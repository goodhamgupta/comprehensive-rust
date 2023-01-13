// from and into facilitate type conversions

fn main() {
    let s = String::from("Hello");
    let addr: std::net::Ipv4Addr = ([127, 0, 0, 1]).into();
    let one = i16::from(true);
    let bigger = i32::from(123i16);
    println!("{s} {addr} {one} {bigger}");
}
// into is auto imlemented when from is implemented
