use std::fs::File;
fn bad_index() {
    let vector = vec![1, 2, 3];
    vector[4]; // panic
    panic!("This is a panic message");
}

fn open_unexisting_file() {
    let f = File::open("unexisting_file.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Problem opening the file: {:?}", error);
            return;
        }
    };

    let metadata = f.metadata().unwrap();
    println!("The file is {} bytes long", metadata.len());
}

fn main() {
    // bad_index();
    open_unexisting_file();
}
