fn main() {
    //TODO: los vectores si pueden modificar su longitud pero como los arreglos solo pueden almacenar un tipo ed valor
    //let <name>:Vec<T> = vac![]; donde vec![] es una macreo que nos crea un vector;
    let mut vector = vec![1, 2, 3];
    println!("EL valor del vector es: {:?}", vector);

    //OPTIMIZE en este caso si se puede agregar elementos
    //TODO: para que esto sea posible el vector tiene que ser mutable asi podemos cambiar su tamaño
    vector.push(4); // los valores se agregan al final del vector
    vector.push(5);
    //TODO otra forma de agreagr es con el metodo insert()
    vector.insert(1, -8);// resive dos valores el indice y el valor a insertar 
    vector.insert(0, 0);// resive dos valores el indice y el valor a insertar 

    //TODO eliminar elemento resive el indice a eliminar 
     vector.remove(vector.len() - 1); // de esta forma eliminamos el ultimo elemento

    //TODO: cambiar el valor de un vector segun su indice

    vector[0] = -10;// cambiamos el primer elemento con el valor -10

    //TODO: optener elementos del vector 

    let primer_elemento = vector[0];
    //let ultimo_elemento = vector[vector.len() - 1];

    //TODO el metodo pop() de los vectores se encarga de retornar y eliminar el ultimo elemento de un vector
    // este metodo nos devuelve un objeto con la estructura option por eso usamos el unwrap()
    let ultimo_elemento = vector.pop().unwrap();


    //FIX como en los arreflos el macro de println! no muestra ese tipo de datos asi que usamos el debug {:?}
    println!("EL valor del vector es: {:?}", vector);
    println!("primer elemento: {} - ultimo elemento: {}", primer_elemento, ultimo_elemento);

    //OPTIMIZE otra forma de crear vectores!
    let mut vectorial = Vec::new();//genera un vector vacio []
    //TODO: como esta vacio hasta que no se inserta el primer dato Rust asume que el primer elemento 
    vectorial.push("hola");
    println!("el valor nuevo es {:?}", vectorial);

    //FIX caundo es bueno usar una macro o una structura macro vec![] estructura Vec::new() en resumen la macro la usaremos caundo sepamos de ante mano los valores que seran cargados y la structura caundo no.
}
