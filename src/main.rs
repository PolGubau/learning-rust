
// const MAX_POINTS: i32 = 100_000;


fn main() {
    prueba_inmutabilidad();
}


fn prueba_inmutabilidad() {
    let mut x = 5;
    x = x*2;
    println!("Hello, world! Value of x is: {}", x);
 }