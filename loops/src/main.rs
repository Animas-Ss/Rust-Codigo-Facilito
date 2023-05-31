fn main() {
    //TODO: ciclo infinito para pararlo control + c
    //OPTIMIZE LOOPS

    //TODO: podemos parar el ciclo con un contador si no tiene contador es infinito

    let mut contador = 0;
    //palabra reservada loop con un bloque de codigo
    loop{
        contador += 1;

        println!("Hello, world! {}", contador);
        if contador >= 10 {
            break;//termina el ciclo
        }
    }
}
