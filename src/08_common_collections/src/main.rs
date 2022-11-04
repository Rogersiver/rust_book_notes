
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    let third: Option<&i32> = v.get(2);
    let hundred: Option<&i32> = v.get(100);

    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    match hundred {
        Some(hundred) => println!("the hundreth element is {}", hundred),
        None => println!("There is no hundreth element")
    }

    let mut v2 = vec![100, 32, 57];
    for i in &v2 {
        println!("{}", i);
    }

    for i in &mut v2 {
        *i += 50;
        println!("{}", i);
    }

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

    let data = "initial contents";

    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s is {}", s);

    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{:?}", score);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let mut v = vec![1, 6, 6, 2, 5, 9, 6, 2, 3, 4, 7, 5, 8, 9, 10];
    v.sort();
    let mut counter = HashMap::new();
    for i in &v {
        *counter.entry(i.to_string()).or_insert(0) += 1;
    }
    v.dedup();
    println!("this is v {:?}", v);
    println!("this is vs length {:?}", v.len());
    let vlen = v.len();
    if let 0=vlen%2{
        println!("the median is {:?} and {:?}", v[(vlen / 2) - 1], v[vlen / 2]);

    } else {
        println!("odd");
        println!("the median is {:?}", v[vlen / 2]);
    }

    println!("this is counter {:?}", &counter);

    fn find_max(map: &mut HashMap<String, i32>) -> i32 {
        let mut max = 0;
        let mut res = 0;
        for(key, value) in &*map {
            if value > &max {
                max = *value;
                res = key.to_string().parse::<i32>().unwrap();
            }
        }
        res
    }
    let mode = find_max(&mut counter);
    println!("the mode is {}", mode);
}
