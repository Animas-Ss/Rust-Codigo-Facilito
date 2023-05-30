

fn main() {
    //OPTIMIZE forma de declarar variables
    //let <nombre de la variable> = <El valor>
    //let <nombre de la variable> : <tipo de variable> = <el valor>
    //let <name>: <T> = <value>
    //let mut <name>: <T> = <value>

    //OPTIMIZE formas de declarar constante con convencion el nombre va en mayusculas
    //const <NAME>: <T> = <valor>
    //static <NAME> = <value>
    /*
    Comentario con multiples lineas, cuando declaramos una variable puede ser mutable o no mutable
     */

    let numero_uno: i32 = 15;//fix recomiendan implementar el tipo
    let numero_dos = 10;//FIX pero determina el lenguaje que tipo es;

    let resultado = numero_uno + numero_dos;

    //TODO formas de imprimir y que figuren las variables 
    println!("el resultado es ({} + {}): {}", numero_uno, numero_dos, resultado);
    println!("el resultado es ({numero_uno} + {numero_dos}): {}", resultado);

    //TODO en Rust por defecto todas las variables on inmtables , para poder hacerlas mutables se usa la palabra reservada mut 
    let mut numero_tres = 10;
    println!("valor inicial de mi variable {}", numero_tres);//fix imprimo el valor inicial luego cambio el valor
    numero_tres = 50;
    println!("nuevo valor de mi variable {numero_tres}");

    //TODO: CONSTANTES
    const VALOR: i32 = 20;
    let resul : i32 = numero_uno + VALOR;
    println!("EL valor de la suma con una constante: {}", resul);
    let total: i32 = numero_uno + numero_dos + numero_tres + VALOR;
    println!("la suma de todas mis variables ({numero_uno} + {numero_dos} + {numero_tres} + {VALOR}) : {}", total);

    //fix las cosntante nunca cambiaran el valor en tiempo de ejecucion 
}
