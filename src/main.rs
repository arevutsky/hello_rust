use bigdecimal:: { BigDecimal, FromPrimitive };

fn main() {
    let a = BigDecimal::from_f32(0.1).unwrap();
    let b = BigDecimal::from_f32(0.2).unwrap();
    println!("Input ({}))", a+b);
}


