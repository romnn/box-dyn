## box-dyn

[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/romnn/box-dyn/build.yml?branch=main&label=build">](https://github.com/romnn/box-dyn/actions/workflows/build.yml)
[<img alt="test status" src="https://img.shields.io/github/actions/workflow/status/romnn/box-dyn/test.yml?branch=main&label=test">](https://github.com/romnn/box-dyn/actions/workflows/test.yml)
[<img alt="crates.io" src="https://img.shields.io/crates/v/box-dyn">](https://crates.io/crates/box-dyn)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/box-dyn/latest?label=docs.rs">](https://docs.rs/box-dyn)

A simple macro, here is how to use it:

```rust
#[box_dyn::box_dyn]
trait MyTrait {
    fn method_a(&self, param: usize) -> String;
    fn method_b(&mut self);
}
```

This will automatically implement `MyTrait` for `Box<T: MyTrait>`:

```rust
impl<T> MyTrait for Box<T> where T: MyTrait {
    fn method_a(&self, param: usize) -> String {
        MyTrait::method_a(self.as_ref(), param)
    }
    fn method_b(&mut self) {
        MyTrait::method_b(self.as_mut())
    }
}
```
