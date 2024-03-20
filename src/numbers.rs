use crate::trit::Trit;

pub struct trit_u8{
    values: Vec<Trit>
}

impl trit_u8{
    fn new() -> trit_u8{
        trit_u8{
            values: vec![Trit::F,Trit::F,Trit::F,Trit::F,Trit::F,Trit::F,Trit::F,Trit::F],
        }
    }
}