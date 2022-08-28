use std::fs;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    for file_dir in fs::read_dir("./documentos").unwrap() {
        let mut f = File::open(file_dir.unwrap().path().as_path())?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        println!("{}", contents);
    }
    Ok(())
}