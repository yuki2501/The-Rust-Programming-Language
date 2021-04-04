use std::collections::HashMap;
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    println!("{:?}", v);
    let v = vec![100,32,57];
    for i in &v {
        println!("{}",i);
    }
    let mut v = vec![100,32,57];
    for i in &mut v {
        *i += 50;
    }
    
    println!("{:?}",v);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("hello: {:?}", hello);
    println!("hello[0..4]: {:?}", s);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}
