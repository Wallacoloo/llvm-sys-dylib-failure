extern crate libloading;

fn main() {
    libloading::Library::new("./libllvm_so.so").unwrap();
}

