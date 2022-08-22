pub mod baz;

pub fn foo_func() {
    println!("hello foo");
}

pub mod bar {
    pub fn bar_func() {
        println!("hello bar");
    }
}
