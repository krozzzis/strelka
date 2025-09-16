#[derive(Default, Debug, Clone)]
pub enum Value {
    #[default]
    None,
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    List(Vec<Value>),
}
