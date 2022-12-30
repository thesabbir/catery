use std::fs;
use std::env;

fn catey(file_path: &str) -> String {
    // construct file path
     let contents = fs::read_to_string(file_path)
        .expect("should have been able to read the file");
    println!("{contents}");
    return file_path.to_string()
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: catey <file_path>");
        return;
    }
    let file_path = &args[1];
    catey(file_path);
}


#[cfg(test)]
mod tests {
    use crate::catey;

    #[test]
    fn it_works() {
        let file_path = "samples/hello.txt";
        let result = catey(file_path);
        assert_eq!(result, file_path);
    }
    #[test]
    #[should_panic="should have been able to read the file"]
    fn it_fails() {
        let file_path = "samples/hello_none.txt";
        let result = catey(file_path);
        assert_eq!(result, file_path);
    }
}