use std::fmt::Display;

pub trait RdbcSQL {
    fn to_sql(&self) -> String;
}
impl<T> RdbcSQL for T where T: ToString {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
