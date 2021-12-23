use enums::data::MyEnum;
use enums::data::MyEnumKind;
use enums::DerivedStruct;

fn main() {
  let derived = DerivedStruct {};
  println!("Hello {:?}", MyEnumKind::Alpha);
}
