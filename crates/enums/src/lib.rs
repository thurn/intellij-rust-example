use macros::MyDerive;

pub mod data;

#[derive(Debug, MyDerive)]
pub enum TestMacro {
    Macro
}