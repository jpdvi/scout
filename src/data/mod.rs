use std::collections::HashMap;

pub struct Data<'a> {
    pub data : HashMap<&'a str, &'a str>,
}


impl<'a> Data<'a> {
    pub fn new() -> Data<'a> {
        let mut d = Data {
            data : [].iter().cloned().collect()
        };
        return d
    }

    pub fn add(&mut self, key: &'a str, value: &'a str) { //-> Result<Option<(&str, &str)>, &str> {
        self.data.insert(key, value);        
    }

    pub fn add_many(&mut self, key_values: Vec<(&'a str, &'a str)>) {
        for kv in key_values.iter() {
            self.add(kv.0, kv.1);
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
