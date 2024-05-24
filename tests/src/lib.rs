#[cfg(test)]
mod tests {
    #![allow(unused)]

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
}
