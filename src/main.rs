pub mod datastruct;

fn main() {
    println!("Hello, world!");
    crate::datastruct::Error::print(&crate::datastruct::Error::new(255, "Unspecified Error".parse().unwrap()))
}
