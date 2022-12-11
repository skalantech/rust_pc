fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(5);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v1 = vec![100, 32, 57];
    for i in &v1 {
        println!("{}", i);
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50;
        println!("{}", i);
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{}: {}", team_name, score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    println!("{:?}", score);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{:?}", map);

    let text = "hello world wonderful world";

    let mut map1 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map1.entry(word).or_insert(0);
        println!("{count}");
        *count += 1;
        println!("{count}");
    }

    println!("{:?}", map1);
}
