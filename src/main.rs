use std::collections::HashMap;

fn main() {
    let row_string = r#"
        // Comment here
        key: value

        key2: value2
        // Another comment
        key3: value3
        "#;

    let hashmap = translate_to_hashmap(row_string.to_string());

    for (key, value) in hashmap.iter() {
        println!("{}: {}", key, value);
    }
}

fn translate_to_hashmap(row_string: String) -> HashMap<String, String> {
    let mut lines = get_filtered_lines(&row_string);
    get_hashmap_from(&mut lines)
}

fn get_filtered_lines(row_string: &String) -> Vec<String> {
    let mut lines = vec![];

    for line in row_string
        .split("\n")
        .into_iter()
        .filter(|l| l.contains(":"))
    {
        lines.push(line.trim().to_string());
    }
    lines
}

fn get_hashmap_from(lines: &mut Vec<String>) -> HashMap<String, String> {
    let mut hashmap = HashMap::<String, String>::new();

    for line in lines.iter() {
        let pair: Vec<&str> = line.split(":").collect();

        hashmap.insert(
            match pair.get(0) {
                Some(key) => key.trim().to_string(),
                None => "".to_string(),
            },
            match pair.get(1) {
                Some(value) => value.trim().to_string(),
                None => "".to_string(),
            },
        );
    }
    hashmap
}
