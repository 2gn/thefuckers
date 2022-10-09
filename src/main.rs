mod corrector;

fn main() {
    println!(
        "{}", 
        corrector::correct("cs ~/Documents".to_string())
    )
}