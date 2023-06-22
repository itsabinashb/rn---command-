use std::fs;

pub fn rename(_name: &str, _path: &str) -> String {
    let mut name = "";
    let mut path = "";
    if let Some(last_slash_index) = _path.rfind('/') {
       path = &_path[..last_slash_index];
       name= &_path[last_slash_index+1..];
    }else{
        println!("Err: Please provide proper path with slash")
    }
    match fs::rename(format!("{}/{}", path, name), format!("{}/{}", path, _name)) {
        Ok(()) => println!("renamed"),
       Err(e) => println!("{e}"),
    }
    format!("{}/{}", path, _name)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn compare(){
        assert_eq!("./test/new.txt", rename(&String::from("new.txt"), &String::from("./test/old.txt")));
    }
}