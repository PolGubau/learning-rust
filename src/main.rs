
// const MAX_POINTS: f32 = 100.43;

fn main() {
    let result = calcular_factorial(25);
    println!("Resultado es {}",result)
}


fn calcular_factorial(number:u128)->u128{

    if number == 0 || number ==1{
        1
    } else{
        let mut result = number;
        for i in (0..result).rev(){
            result = result*i;
        }
        result
    }
}