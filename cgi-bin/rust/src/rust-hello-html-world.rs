use std::env;
use chrono::Local;

fn main() {
    let now = Local::now();
    let current_time = now.format("%a %b %d %H:%M:%S %Y");
    let remote_addr = env::var("REMOTE_ADDR").unwrap_or_else(|_| "Unknown".to_string());

    // Print HTTP header
    println!("Cache-Control: no-cache");
    println!("Content-type: text/html\n");

    // Print HTML body
    println!(
        "<html><head><title>Hello CGI World</title></head>\
        <body><h1 align=center>Hello Rust HTML World</h1>\
        <hr/>\n"
    );
    println!("Hello World<br/>");
    println!("This program was generated by the Rust programming language.<br/>");
    println!("This program was generated at: {}<br/>", current_time);
    println!("Your current IP address is: {}<br/>", remote_addr);

    // Print HTML footer
    println!("</body></html>");
}