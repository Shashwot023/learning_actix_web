use std::ops::Deref;

#[derive(Debug)]
struct CustomSmartPointer<T: std::fmt::Display> {
    data: T,
}

impl<T: std::fmt::Display> CustomSmartPointer<T> {
    fn new(x: T) -> CustomSmartPointer<T> {
        CustomSmartPointer { data: x }
    }
}

impl<T: std::fmt::Display> Deref for CustomSmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: std::fmt::Display> Drop for CustomSmartPointer<T> {
    fn drop(&mut self) {
        println!("Dropping CSP data: `{}`!", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let y = CustomSmartPointer::new(String::from("Visitor"));
    hello(&y);
    println!("{y:?}");
}

