use std::collections::HashMap;

pub fn translate_to_hashmap(row_string: String) -> Option<HashMap<String, String>> {
    let mut hashmap = HashMap::new();

    for line in row_string
        .split("\n")
        .into_iter()
        .filter(|l| l.contains(":"))
        .into_iter()
    {
        let pair: Vec<&str> = line.split(":").collect();
        if pair.len() != 2 {
            return None;
        }
        hashmap.insert(
            pair.get(0).unwrap().trim().to_string(),
            pair.get(1).unwrap().trim().to_string(),
        );
    }
    Some(hashmap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common() {
        let row_string = r#"
            // Comment here
            key: value
            key2: value2
            // Another comment
            key3: value3
        "#;
        let hashmap = translate_to_hashmap(row_string.to_string()).unwrap();

        assert_eq!(hashmap.get("key").unwrap(), "value");
        assert_eq!(hashmap.get("key2").unwrap(), "value2");
        assert_eq!(hashmap.get("key3").unwrap(), "value3");
    }
}
