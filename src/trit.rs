use crate::half_adder;

pub enum Trit{
    T,
    F,
    Z(f32)
}

impl Trit{
    pub fn from_boolean(source: bool) -> Trit{
        return match(source){
            true => Trit::T,
            false => Trit::F
        };
    }

    pub fn from_float(source: f32) -> Option<Trit>{
        return match(source){
            1.0 => Some(Trit::T),
            0.0 => Some(Trit::F),
            x if 0.0 < x && x < 1.0 => Some(Trit::Z(x)),
            _ => None
        };
    }

    pub fn from_float_unchecked(source: f32) -> Trit{
        return match(source){
            1.0 => Trit::T,
            0.0 => Trit::F,
            x if 0.0 < x && x < 1.0 => Trit::Z(x),
            _ => unreachable!()
        };
    }

    pub fn to_float(&self) -> f32{
        return match(self){
            Trit::T => 1.0,
            Trit::F => 0.0,
            Trit::Z(x) => x
        };
    }

    pub fn not(&self) -> Trit{
        return Trit::from_float_unchecked(1.0 - self.to_float())
    }

    pub fn and(&self, other: &Trit) -> Trit{
        return Trit::from_float_unchecked(self.to_float() * other.to_float())
    }

    pub fn or(&self, other: &Trit) -> Trit{
        return self.not().and(&other.not()).not()
    }

    pub fn xor(&self, other: &Trit) -> Trit{
        //or and (not and)
        return self.or(other).and(&self.nand(other))
    }

    pub fn nor(&self, other: &Trit) -> Trit{
        return self.or(other).not()
    }

    pub fn nand(&self, other: &Trit) -> Trit{
        return self.and(other).not()
    }

    pub fn doubt(&self) -> bool{
        return match(self){
            Trit::T => true,
            _ => false
        }
    }

    pub fn assume(&self) -> bool{
        return match(self){
            Trit::F => false,
            _ => true
        }
    }

    pub fn round(&self) -> bool{
        return match (self) {
            Trit::T => true,
            Trit::F => false,
            Trit::Z(x) if *x <0.5 => false,
            Trit::Z(x) if *x >=0.5 => true,
            _ => unreachable!()
        }
    }
}

type TritByte = [Trit;8];

fn byte_or(first: &TritByte, second: &TritByte) -> TritByte{
    first.iter()
        .zip(second)
        .map(|(first, second)|first.or(second))
        .into()
}

fn byte_and(first: &TritByte, second: &TritByte) -> TritByte{
    first.iter()
        .zip(second)
        .map(|(first, second)|first.and(second))
        .into()
}

fn byte_not(first: &TritByte) -> TritByte{
    first.iter()
        .map(|first|first.not())
        .into()
}

fn full_add(a: &Trit, b: &Trit, c: &Trit) -> (Trit, Trit){
    let z0 = a.xor(b);
    let sum = c.xor(&z0);
    let z1 = z0.and(c);
    let z2 = a.and(b);
    let carry = z1.or(&z2);
    (sum, carry)
}

fn byte_full_add(first: &TritByte, second: &TritByte) -> (TritByte, Trit){
    let mut result = vec![];
    let mut carry_trit = Trit::F;
    for i in 0..=7{
        let result = full_add(&first[i], &second[i], &carry_trit);
        carry_trit = result.1;
        result.push(result.0);
    }
    (result.into(), carry_trit)
}

fn byte_from_u8(source: u8) -> TritByte{
    let mut result: Vec<Trit> = vec![];
    for i in 0..=7{
        let bitval = match(source>>i)&1{
            1 => Trit::T,
            0 => Trit::F,
            _ => unreachable!()
        };
        result.push(bitval);
    }
    result.into()
}

struct TritString{
    values: Vec<Trit>
}
impl TritString{
    fn bit_or(&self, other: &TritString) -> TritString{
        TritString{ values: vec![] }
    }
}