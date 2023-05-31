fn main() {
    //TODO ciclo while
/* 
    let mut contador: i32 = 1;

    while contador <= 10{

        println!("Contador: {}", contador);

        contador += 1;
    } */

    //123 -> debe decirme que mmi digito es 3
    //12345 -> debe decirme 5
    //123456789 -> debe decirme 9
    

    //para concer los digitos de un numero podemos dividirlo en 10
    //while depende de una condicion
    //TODO el ciclo while se usa caundo no sabemos cuantas iteraciones aremos , como el siguiente ejemplo
    let mut numero: i32 = 123456789;
    let mut contador_dos = 0;

    while numero > 0 {
        numero = numero / 10;
        contador_dos += 1;
    }

    println!("La cantidad de digitos son: {}", contador_dos);
    
}
