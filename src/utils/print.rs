
pub fn info<T: ToString + ?Sized>(label: &str, message: Vec::<&T>) {
    let mut result = String::new();
    result.push_str("\u{1b}[1;34m");
    result.push_str(label);
    result.push_str(" \u{1b}[0;00m");
    for item in message.iter() {
        result.push_str(item.to_string().as_str());
    }
    println!("{}", result);
}
pub fn warning<T: ToString + ?Sized>(label: &str, message: Vec::<&T>) {
    let mut result = String::new();
    result.push_str("\u{1b}[1;33m");
    result.push_str(label);
    result.push_str(" \u{1b}[0;00m");
    for item in message.iter() {
        result.push_str(item.to_string().as_str());
    }
    println!("{}", result);
}
pub fn error<T: ToString + ?Sized>(label: &str, message: Vec::<&T>) {
    let mut result = String::new();
    result.push_str("\u{1b}[1;31m");
    result.push_str(label);
    result.push_str(" \u{1b}[0;00m");
    for item in message.iter() {
        result.push_str(item.to_string().as_str());
    }
    println!("{}", result);
}

