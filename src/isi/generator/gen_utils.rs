use crate::isi::ast::ast::DataType;

pub fn gen_proper_type_code(value: &str, data_type: DataType) -> String {
    match data_type {
        DataType::String => {
            format!("\"{}\"", value)
        }
        DataType::Int => String::from(value),
        _ => {
            todo!("gen_proper_type_code() not yet implemented for {data_type:?}")
        }
    }
}
