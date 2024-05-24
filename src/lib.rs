#[cfg(test)]
mod tests {
    #![allow(unused)]

    #[test]
    fn it_works() {
        // #[derive(box_dyn::BoxDyn)]
        #[box_dyn::box_dyn]
        trait SmallTrait {
            fn finalize(self) -> String;
        }

        // #[derive(box_dyn::BoxDyn)]
        #[box_dyn::box_dyn]
        trait BigTrait {
            fn func<'a>(&self, value: &'a str) -> &str;
            fn another(&self, help: String);
            fn mutate(&mut self);
        }

        mod other {
            // #[derive(box_dyn::BoxDyn)]
            #[box_dyn::box_dyn]
            pub trait OtherTrait {
                fn say_hi(&self);
            }
        }

        // box_dyn::impl_trait!(BigTrait);
        // box_dyn::impl_trait!(SmallTrait, other::OtherTrait);
    }
}
