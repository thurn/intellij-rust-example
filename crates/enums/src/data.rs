use strum_macros::EnumDiscriminants;
 
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, EnumDiscriminants)]
#[strum_discriminants(name(MyEnumKind))]
pub enum MyEnum {
    Alpha,
    Bravo,
    Charlie
}