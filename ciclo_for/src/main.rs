fn main() {
    //TODO ciclo for podemos iterar arreglos
    /* let numeros: [i32; 5] = [1, 2, 3, 4, 5]; */
    //OPTIMIZE es como un ciclo foreach() iteramos dentro de nuestro arreglo y conocemos los valores
    /* for numero in numeros.iter() {
        println!("el valor de numero: {}", numero);
    } */
    //TODO podemos iterar por rangos , los rangos funcionan el ultimo numero puesto menos 1 
    //TODO en este ejemplo se imprime del 1 al 99 no se considera el 100 si lo deseamos mostrar nuestro rango deberia de ser 1..101
    /* for numero_dos in 1..100 {
        println!("{}", numero_dos);
    } */

    //FIZZ BUZZ
    for numero in 1..101 {
        if numero % 3 == 0 && numero % 5 == 0 {
            println!("Fizz Buzz");
        }else if numero % 3 == 0 {
            println!("Fizz");
        }else if numero % 5 == 0 {
            println!("Buzz");
        }else{
            println!("{}", numero);
        }
    }

}
