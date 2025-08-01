use std::fs::File;

use lightswitch_object::ObjectFile;

fn main() {
    let of = ObjectFile::new(&File::open("test.lso").unwrap()).unwrap();
    println!("ID: {}", of.id().unwrap());
}
