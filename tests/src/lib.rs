#[cfg(test)]
mod tests {
    #![allow(unused)]
    use color_eyre::eyre;

    #[test]
    fn it_works() {
        #[box_dyn::box_dyn]
        trait SmallTrait {
            fn finalize(self) -> String;
        }

        #[box_dyn::box_dyn]
        trait BigTrait {
            fn func<'a>(&self, value: &'a str) -> &str;
            fn another(&self, help: String);
            fn mutate(&mut self);
        }

        mod other {
            #[box_dyn::box_dyn]
            pub trait OtherTrait {
                fn say_hi(&self);
            }
        }
    }

    #[test]
    fn two_params() {
        #[derive(Default)]
        struct Array<T> {
            inner: Vec<T>,
        }

        #[box_dyn::box_dyn]
        pub trait MyOtherTrait {
            fn is_mine(&self) -> bool;
        }

        #[box_dyn::box_dyn]
        pub trait Warper: std::fmt::Debug + MyOtherTrait {
            fn warp(
                &self,
                image: String,
                k: Array<f32>,
                r: Array<f32>,
                // k: old
                // r: old
            ) -> eyre::Result<(String, String)>;
        }

        #[derive(Debug, PartialEq)]
        struct MyWarper {}

        impl MyOtherTrait for MyWarper {
            fn is_mine(&self) -> bool {
                true
            }
        }

        impl Warper for MyWarper {
            fn warp(
                &self,
                image: String,
                k: Array<f32>,
                r: Array<f32>,
                // k: old
                // r: old
            ) -> eyre::Result<(String, String)> {
                Ok(("a".to_string(), "b".to_string()))
            }
        }

        let boxed_my_warper: Box<MyWarper> = Box::new(MyWarper {});
        assert_eq!(
            boxed_my_warper
                .warp("".to_string(), Array::default(), Array::default())
                .ok(),
            Some(("a".to_string(), "b".to_string()))
        );

        let boxed_my_warper: Box<dyn Warper> = Box::new(MyWarper {});
        assert_eq!(
            boxed_my_warper
                .warp("".to_string(), Array::default(), Array::default())
                .ok(),
            Some(("a".to_string(), "b".to_string()))
        );
    }
}
