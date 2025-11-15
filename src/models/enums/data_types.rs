use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive)]
pub enum DataType {
    Data = 0x0,
    String = 0x1,
    Integer = 0x4,
}