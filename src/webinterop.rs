



#[cfg(test)]
mod tests {
    use webinterop::web_interop;

    #[web_interop]
    struct TestStruct {

        #[property(rename = "field")]
        field: i32,
    }

    impl TestStruct {
        fn add(&self, other: i32) -> i32 {
            self.field + other
        }
    }

    #[test]
    fn test() {
        let test = TestStruct {
            field: 3
        };

        let k = test.add(5);
    }
}