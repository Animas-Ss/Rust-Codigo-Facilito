use std::io;

fn main() {
    let mut color = String::new();

    println!("ingresa el color para el semaforo:");

    io::stdin().read_line(&mut color).unwrap();//unwrap sirve para poder tomar el resultado

    let color: String = color.trim().to_lowercase();//Para pasar al respuesa a minusculas y controlar mejor los errores

    //TODO: las condiciones en Rust no llevan parentecis
    if color == "verde" {
        println!("Puede continuar");
    }else if color == "amarillo" {
        println!("Disminulla la velocidad");
    }else if color == "rojo" {
        println!("Frene hasta que la luz sea verde");
    }else {
        println!("el color no es niguno de los solicitados")
    }
}
