use std::collections::HashMap;

fn main() {
    let row_string = r#"
        // Comment here
        key: value

        key2: value2
        // Another comment
        key3: value3
        "#
    .to_string();

    let mut lines = vec![];

    for line in row_string
        .split("\n")
        .into_iter()
        .filter(|l| l.contains(":"))
    {
        lines.push(line.trim().to_string());
    }
    let mut result = HashMap::<String, String>::new();

    for line in lines.iter() {
        let v: Vec<&str> = line.split(":").collect();
        let a = match v.get(0) {
            Some(key) => key,
            None => "",
        };
        let b = match v.get(1) {
            Some(val) => val,
            None => "",
        };
        result.insert(a.trim().to_string(), b.trim().to_string());
    }
}
