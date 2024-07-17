fn main() {
    // Strings are stored as a collection of UTF-8 encoded bytes

    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    // Appending to a string
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    let str1 = String::from("Hello");
    let str2 = String::from(", World!");
    let str3 = format!("{}{}", str1, str2);

    // Indexing
    let hello = String::from("Hello");


    for b in "नमस्ते".bytes() {
        println!("{}", b); // print as bytes
    }

    for c in "नमस्ते".chars() {
        println!("{}", c); // scalar values
    }
}