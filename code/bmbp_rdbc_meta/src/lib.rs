mod bean;
mod dml;
mod dql;
pub use bean::*;
pub use dml::*;
pub use dql::*;
#[cfg(test)]
mod test {
    use crate::bean::RdbcColumnVo;

    #[test]
    pub fn test_bean() {
        let column = RdbcColumnVo::default();
        println!("{:?}", column);
    }
}
