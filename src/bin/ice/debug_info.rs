pub fn print_source_info(file_name: &str, source_code: &str) {
    println!("Source code from {file_name}:");
    println!("{}", String::from("-").repeat(100));
    println!("{source_code}");
    println!("{}", String::from("-").repeat(100));
}
