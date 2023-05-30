fn main() {
    //TODO  A DIFERENCIA DE LOS ARREGLOS LAS TUPLAS PUEDEN ALMACENAR VARIOS VALORES DE TIPOS DIFERENTES
    //let <name> = (<T>, <T>, <T>);
    //let <name>: (<tipo>, <tipo>, <tipo>) = (<T>, <T>, <T>);
    //OPTIMIZE las tuplas se declara con parentecis los indices al igual que los arrelos empeizan en 0
    let tupla = (10, false, 5.5);
    println!("El valor de la tupla es : {:?}", tupla);
    let mut tupla: (i32, bool, f64, i32) = (10, false, 5.5, 30);
    //FIX como en los arreglos el macro de println! no puede ser usado para ver los valores de la tupla
    //FIX por lo cual vamos a usar la sintaxis para debugear {:?}
    println!("El valor de la tupla es : {:?}", tupla);
    //TODO: para acceder a los elementos de una tupla lo hacemos con la siguiente sintaxis
    //<nombre>.<index>
    let primer_elemento = tupla.0;
    //los valores de la tupla tambien se peuden modificar siempre que la tupla sea mutable (mut)
    tupla.1 = true;
    let ultimo_elemento = tupla.3;

    println!("El primer elemento: {}", primer_elemento);
    println!("El segundo elemento: {:?}", tupla);
    println!("El ultimo elemento: {}", ultimo_elemento);

    //FIX  IMPORTANTE LAS TUPLAS NO PUEDEN INCREMENTAR O DECRECER SU LONGITUD

}
