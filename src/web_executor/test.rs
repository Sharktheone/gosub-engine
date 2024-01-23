use alloc::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Add;

use webinterop::{web_fns, web_interop};

use crate::types::Result;
use crate::web_executor::js::{Args, JSContext, JSFunction, JSFunctionCallBack, JSObject, JSValue, ValueConversion, VariadicArgs};
use crate::web_executor::js::v8::{V8Context, V8Function, V8Value};

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
    fn variadic<T: VariadicArgs>(nums: T) {}

    fn v8_variadic(nums: V8Value) {}
}

// #[test]
// fn test() {
//     let test = TestStruct {
//         field: 3,
//         field2: HashMap::new(),
//     };
//
//     let k = test.add(5);
// }


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


struct Test2 {
    field: i32,
    other_field: String,
}


impl Test2 {
    fn cool_fn(&self) -> i32 {
        self.field
    }

    fn add(&mut self, other: i32) {
        self.field += other;
    }

    fn concat(&self, other: String) -> String {
        self.other_field.clone() + &other
    }

    fn takes_ref(&self, other: &String) -> String {
        self.other_field.clone() + other
    }

    fn variadic<T: VariadicArgs>(nums: T) {
        for _ in nums {
            println!("got an arg...");
        }
    }
}


struct TestStorage {
    test: Test2,
}


impl Test2 {
    fn implement(s: Rc<RefCell<Self>>, mut ctx: V8Context) -> Result<()> {
        let obj = ctx.new_global_object("Test2")?; //#name


        let cool_fn = {
            let s = Rc::clone(&s);
            V8Function::new(ctx.clone(), move |cb| {  //TODO: add R::Function::new
                let num_args = 0; //function.arguments.len();
                if num_args != cb.len() {
                    // cb.error("wrong number of arguments"); //TODO
                    return;
                }

                let ctx = cb.context();

                let Ok(ret) = s.borrow().cool_fn().to_js_value(ctx.clone()) else {
                    // cb.error(e); //TODO
                    return;
                };

                cb.ret(ret);
            })?
        };

        obj.set_method("cool_fn", &cool_fn)?;


        let add = {
            let s = Rc::clone(&s);
            V8Function::new(ctx.clone(), move |cb| {
            let num_args = 1; //function.arguments.len();
            if num_args != cb.len() {
                // cb.error("wrong number of arguments"); //TODO
                return;
            }

            let ctx = cb.context();

            let args = cb.args();

            // let Some(arg0) = cb.args().get(0, ctx.clone()). else {
            //     // cb.error("failed to get argument"); //TODO
            //     return;
            // };

            let num = cb.args().get(0, ctx.clone()).unwrap().as_number().unwrap();

            // let Ok(arg0) = arg0.as_number() else {
            //     // cb.error("failed to convert argument"); //TODO
            //     return;
            // };

            // let arg0 = arg0 as i32;

            // let arg0 = arg0.clone().add(0);

            let ret = s.borrow_mut().add(num as i32).to_js_value(ctx.clone()).unwrap();


            cb.ret(ret);
        })?
        };
        Ok(())
    }
}

