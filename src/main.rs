pub enum B {
    B(u8),
}

enum A {
    A(B),
}

fn main() {
    let a = A::A(B::B(100));

    println!("Hello world!");
}
