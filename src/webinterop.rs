



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use webinterop::{web_fns, web_interop};

    #[web_interop]
    struct TestStruct {

        #[property]
        field: i32,

        #[property]
        field2: HashMap<i32, i32>, //should crash it
    }

    #[web_fns]
    impl TestStruct {

        fn add(&self, other: i32) -> i32 {
            self.field + other
        }
    }

    #[test]
    fn test() {
        let test = TestStruct {
            field: 3,
            field2: HashMap::new(),
        };

        let k = test.add(5);
    }
}