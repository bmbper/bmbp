mod dsl_struct;
mod dsl_func;
mod dsl_trait;
mod dsl_value;

pub use dsl_struct::*;
pub use dsl_func::*;
pub use dsl_trait::*;
pub use dsl_value::*;

#[cfg(test)]
mod test {
    #[test]
    fn test_table() {
    }
}