pub use date::date_time_now;
pub use id::simple_uuid;
pub use id::simple_uuid_upper;
pub use id::uuid;
pub use id::uuid_upper;
pub use string::*;
pub use tree::TreeBuilder;
pub use value::insert_decorate;
pub use value::update_decorate;

mod bmbp_value;
pub mod crypto;
mod date;
mod id;
pub mod number;
mod string;
mod tree;
pub mod url;
mod value;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
