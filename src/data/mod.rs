use std::collections::HashMap;

pub struct Data {
    pub data : HashMap<String, String>,
}


impl Data {
    pub fn new() -> Data{
        let mut d = Data {
            data : [].iter().cloned().collect()
        };
        return d
    }

    pub fn add(&mut self, key: &str, value: &str) { //-> Result<Option<(&str, &str)>, &str> {
        self.data.insert(String::from(key), String::from(value));        
    }

    pub fn add_many(&mut self, key_values: Vec<(&str, &str)>) {
        for kv in key_values.iter() {
            self.add(kv.0, kv.1);
        }
    }
}

impl Clone for Data {
    fn clone(&self) -> Self {
        let mut new_map : HashMap<String, String> = [].iter().cloned().collect();
        for (i, item) in self.data.keys().enumerate() {
            new_map.insert(String::from(item), String::from(&self.data[item]));
        }
        Self {
            data : new_map
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn test_create_data() {
       let data = Data::new(); 
    }

    #[test]
    fn test_add_data() {
       let mut data = Data::new();
       data.add("hello", "world");
       assert_eq!(data.data.contains_key("hello"), true);
       assert_eq!(data.data["hello"], "world");
    }

    #[test]
    fn test_add_many_data() {
        let mut data = Data::new();
        data.add_many(vec![("one", "two"), ("two", "three"), ("three", "four")]);
        assert_eq!(data.data.contains_key("one"), true);
        assert_eq!(data.data.contains_key("two"), true);
        assert_eq!(data.data.contains_key("three"), true);

        assert_eq!(data.data["one"], "two");
        assert_eq!(data.data["two"], "three");
        assert_eq!(data.data["three"], "four");

    }
}
