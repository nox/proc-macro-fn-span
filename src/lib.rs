// This is a very silly proc macro that generates a copying getter for all
// fields of a struct, it uses spans to properly tie the definition of the
// getter to the location of the field it returns.
use get::GetCopy;

#[derive(GetCopy)]
pub struct Struct {
    f1: bool,
    f2: &'static str,
}

impl Struct {
    pub fn as_parts(&self) -> (bool, &'static str) {
        let f1 = self.f1();
        let f2 = self.f2();

        (f1, f2)
    }
}
