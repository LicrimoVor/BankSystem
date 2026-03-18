#[derive(Debug, Clone, Copy)]
pub struct PositiveInteger(u32);

impl PositiveInteger {
    pub fn new(value: u32) -> Option<Self> {
        if value > 0 { Some(Self(value)) } else { None }
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }

    pub fn multiply(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}

fn main() {
    let a = PositiveInteger::new(5).unwrap();
    let b = PositiveInteger::new(10).unwrap();

    let sum = a.add(b);
    println!("Sum: {}", sum.value());

    let product = a.multiply(b);
    println!("Product: {}", product.value());

    let zero = PositiveInteger::new(0);
    assert!(zero.is_none());
}
