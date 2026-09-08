use std::fmt::Display;

pub fn success(message: impl Display) {
    println!("[+] {message}");
}
pub fn info(message: impl Display) {
    println!("[*] {message}");
}
pub fn warning(message: impl Display) {
    eprintln!("[!] {message}");
}
pub fn unknown(message: impl Display) {
    eprintln!("[?] {message}");
}
pub fn error(message: impl Display) {
    eprintln!("[-] {message}");
}
