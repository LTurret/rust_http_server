use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("key1", "value1");

    println!("{:?}", map);
}
