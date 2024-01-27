use std::collections::HashMap;

pub enum RdbcValue{
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    IntSize(isize),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    UIntSize(usize),
    Float32(f32),
    Float64(f64),
    String(String),
    Date(String),
    Time(String),
    DateTime(String),
    Bool(bool),
    Null,
}