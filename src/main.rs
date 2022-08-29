use std::collections::HashMap;

fn main() {
    let row_string = r#"
        // Comment here
        key: value

        key2: value2
        // Another comment
        key3: value3
        "#;

    let hashmap = translate_to_hashmap(row_string.to_string()).unwrap();

    for (key, value) in hashmap.iter() {
        println!("{}: {}", key, value);
    }
}

fn translate_to_hashmap(row_string: String) -> Option<HashMap<String, String>> {
    let lines = filter_lines(&row_string);
    let mut hashmap = HashMap::new();

    for line in lines.iter() {
        let pair: Vec<&str> = line.split(":").collect();
        if pair.len() > 2 {
            return None;
        }
        hashmap.insert(
            pair.get(0).unwrap().trim().to_string(),
            pair.get(1).unwrap().trim().to_string(),
        );
    }
    Some(hashmap)
}

fn filter_lines(row_string: &String) -> Vec<String> {
    let mut lines = vec![];

    for line in row_string
        .split("\n")
        .into_iter()
        .filter(|l| l.contains(":"))
    {
        lines.push(line.to_string());
    }
    lines
}
