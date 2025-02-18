fn main() {
    let result = calcular_factorial(3);
    println!("Resultado es {}", result);

    let es_el_cuatro_primo = es_primo(4);
    println!("El 4 es primo? {} ", es_el_cuatro_primo)
}

fn calcular_factorial(number: u128) -> u128 {
    if number == 0 || number == 1 {
        1
    } else {
        let mut result = number;
        for i in (1..result).rev() {
            result = result * i;
        }
        result
    }
}

fn es_primo(number: u128) -> bool {
    let mut es_primo = true;
    let num: f64 = number as f64;
    if number > 1 {
        for i in 2..((num.sqrt() as i128) + 1) {
            if num as i128 % i == 0 {
                es_primo = false;
                break;
            }
        }
    }
    es_primo
}
