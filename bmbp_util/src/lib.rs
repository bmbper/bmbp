pub use tree::TreeBuilder;

pub mod crypto;
pub mod id;
pub mod number;
pub mod string;
mod tree;
pub mod url;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
