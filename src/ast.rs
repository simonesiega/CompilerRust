#[derive(Debug, Clone)]
pub struct OperatorDef {
    pub name: String,          // "assign", "add", ...
    pub params: Vec<String>,   // es. ["self: &mut T", "value: T"]
    pub return_type: String,   // es. "void"
    pub body: String,          // corpo in stringa per ora
}