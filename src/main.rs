use std::env;
use rn::rename;
fn main() {
    let args: Vec<String> = env::args().collect();

    let new_name = &args[1];
    let file_path = &args[2];


    let _renamed = rename(new_name, file_path);
}
