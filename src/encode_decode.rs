pub mod encode_decode {
    use std::collections::HashMap;
    use std::string::String;

    pub fn encode(mapping: HashMap<String, i32>, vec: Vec<String>) -> Vec<i32> {
        let mut num: Vec<i32> = Vec::new();
        for s in vec {
            num.push(*mapping.get(&s).unwrap());
        }
        num
    }

    pub fn decode(mapping: HashMap<String, i32>, vec: Vec<i32>) -> Vec<String> {
        let mut strings: Vec<String> = Vec::new();
        for num in vec {
            strings.push(mapping.iter().find_map(|(key, &val)| if val == num {Some(key)} else {None}).unwrap().to_string());
        }
        strings
    }
}