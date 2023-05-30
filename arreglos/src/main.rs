fn main() {
    //OPTIMIZE : podemos definir el array que tipo y espacios
    //let <nombre arreglo> = <valores del arrelo>
    //let <nombre arreglo>: [<T>; <size>] = [<valores del arreglo>]
    //TODO: los arreglos se riguen como en los otros lenguajes con indices empeiza en 0 hasta la posicion del elemento final
    let mut numeros = [1, 2, 3, 4, 5];// size -> 5
    let tablas: [i32; 3] = [1, 2, 3];// size -> 3
    //crear arreglos de forma automatica
    //nos genera un array de 10 posiciones con los valores de 5.5 ueden ser valores booleanos o enteros dependiendo de loq ue encesitemos;
    let valores = [5.5; 10];

    //FIX no podemos imprimir el formato de array en el macro println!
    println!("los valores del arreglos son: {:?}", numeros);//{:?} esta estructura nos permite debugear y conocer lo que tienen adentro neustra variables
    println!("los valores del arreglos tablas son: {:?}", tablas);//{:?} esta estructura nos permite debugear y conocer lo que tienen adentro neustra variables
    println!("los valores del arreglos son: {:?}", valores);//{:?} esta estructura nos permite debugear y conocer lo que tienen adentro neustra variables

    //TODO: recorrer un arreglo 
    let primer_elemento = numeros[0];
    let ultimo_elemento = numeros[numeros.len()-1];
    //OPTIMIZE para cambiar los valores de un arreglo tiene que ser mutable por lo cual tiene que tener el prefijo mut
    numeros[1] = 7;

    println!("el primer valor: {}", primer_elemento);
    println!("el primer valor: {}", ultimo_elemento);
    println!("el calor cambio a: {}", numeros[1]);
    println!("nuevos valores de arreglo: {:?}", numeros);

    //FIX importante aclarar que los arreglos en Rust no pueden modificar su tamaño ni agrandar p reducirlo 
}
