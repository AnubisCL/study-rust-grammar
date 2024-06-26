use std::fmt; // 导入 `fmt`

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{real:?} + {imag:?}i", real=self.real, imag=self.imag)
    }
}

fn main() {
    let c1 = Complex { real:3.3, imag:7.2 };
    println!("Display: {}", c1);
    println!("Debug: {:?}", c1);
}