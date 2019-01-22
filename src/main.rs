use std::io::Read;

fn main() {
    let mut config = String::new();
    let config_path = std::env::current_exe().unwrap();
    let config_path = config_path.parent().unwrap();
    let config_path = config_path.join("config.txt");
    let mut file = std::fs::File::open(config_path).expect("open file err");
    file.read_to_string(&mut config).unwrap();

    println!("Configuration loaded successfully:\n---------\n\n{}", config);
}
