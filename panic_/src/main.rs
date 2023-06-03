
//TODO: panic es una funcion macro de Rust que nos permite finalizar el programa si hay algun error inesperado cortando al ejecucion
fn main() {
    println!("Hello, world! linea 4");
    
    println!("Hello, world! linea 6");
    panic!("el programa se finalizo inesperadamente");
    println!("Hello, world! linea 8");

    println!("Hello, world! linea 10");

    println!("Hello, world! linea 12");
}
