#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub struct StackTypes{
    pub selected: i8,
    pub strtype: Option<String>,
    pub inttype: Option<i32>
}
impl StackTypes {
    pub fn new_int(b: i32) -> Self {
        StackTypes {selected: 1, inttype: Some(b), strtype: None}
    }
    pub fn new_str(b: String) -> Self {
        StackTypes {selected: 0, strtype: Some(b), inttype: None}
    }
}