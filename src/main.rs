mod lib;
use lib::linux;

fn main() {
    println!("{}",linux::pwd());
    let pwd = linux::pwd();
    let mut my_path = std::path::PathBuf::from(pwd);

    loop{
        linux::take_input(&mut my_path);
        println!("{}",my_path.display());
    }
}
