#![allow(unused, unused_imports, unused_variables, dead_code)]
#[allow(dead_code, unused)]
pub mod js;
pub mod interop;


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use webinterop::{web_fns, web_interop};
    use crate::web_executor::js::{JSValue, VariadicArgs};
    use crate::web_executor::js::v8::V8Value;

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

        fn add2(&mut self, other: i32) {
            self.field += other
        }

        fn add3(a: i32, b: i32) -> i32 {
            a + b
        }
        fn variadic<T: VariadicArgs>(nums: T) {

        }

        fn v8_variadic(nums: V8Value) {

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


    fn array_test() {
        let mut test_vec = vec![1, 2, 3];

        vec(test_vec.clone()); //clone only needed for the test

        ref_vec(&test_vec);

        mut_vec(&mut test_vec);

        ref_slice(&test_vec);

        mut_slice(&mut test_vec);

        size_slice(<[i32; 3]>::try_from(test_vec.clone()).unwrap()); //clone only needed for the test

        ref_size_slice(&<[i32; 3]>::try_from(test_vec.clone()).unwrap()); //clone only needed for the test

        mut_size_slice(&mut <[i32; 3]>::try_from(test_vec.clone()).unwrap()); //clone only needed for the test

    }

    fn vec(vec: Vec<i32>) {}
    fn ref_vec(vec: &Vec<i32>) {}

    fn mut_vec(vec: &mut Vec<i32>) {}

    fn ref_slice(slice: &[i32]) {}

    fn mut_slice(slice: &mut [i32]) {}

    fn size_slice(array: [i32; 3]) {}

    fn ref_size_slice(slice: &[i32; 3]) {}

    fn mut_size_slice(slice: &mut [i32; 3]) {}

}
