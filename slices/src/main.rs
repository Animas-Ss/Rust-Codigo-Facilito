fn main() {
    //TODO los Slice son similares a los arreglos , la diferencia es que nos e conoce el tamaño en tiempo de ejecucion
    //Slice -> se almacenan en el Heap
    //Arrays -> se almacenan en el Stack

    //los slice sirven apra optener porciones de un string


    let mensaje: String = String::from("Hola mundo, desde el curso de Rust");

    //TODO como optener un slice
    //el formato seria el indice donde comenzamos dos puntos (..) y el indice donde terminamos [start..end]
    //let hola = &mensaje[0..4];
    //let resto_mensaje = &mensaje[4..mensaje.len()-1];
    //TODO: podemos omitir el indice del principio o el indice del final
    let hola = &mensaje[..4];// agarra desde el principio
    let resto_mensaje = &mensaje[4..];// agarra desde el principio 


    println!("El mensaje es: {}", mensaje);
    println!("el slice es : {}", hola);
    println!("el slice es del resto es : {}", resto_mensaje);
}
