fn main() {
    //TODO: EJEMPLO DE VIDA DE LAS VARIABLES O CODIGO EN UN BLOQUE 
    let mensaje: &str = "Hola, soy una vaiable en el bloque main.";
    {
        //OPTIMIZE una variable que tenga el mismo nombre que la que esta en el bloque main no afecta su estado
        //no se realiza el Shadowing ya que no viven en el mismo scope no destruye la variable , el Shadowing respeta el scope no destruye variables que no esten en el mismo scope
        let mensaje: &str = "Hola, soy una variable en el bloque aninado , scope interno";
        println!("Hola soy un segundo bloque");
        println!("desde el bloque: {}", mensaje);
        //FIX  las variables declaradas de forma global pueden ser usadas en cualquier bloque interno
        //TODO: no asi las internas solo tienen vida en el bloque cuando este termina las mismas son destruidas todas las variables 
        //TODO: sin importal cual 
        let _mensaje_dos = "variable mensaje desde el bloque";// si le pongo el guion bajo no me da error el compilador 
        //OPTIMIZE para que quede claro una variable se crea , se utiliza y se destrulle en un bloque , en este caso tenemos un bloque aninado y la variable mensaje posee un scope mas aplio que mensaje_dos por lo cual puede ser utilizada dentro de bloques internos 
    }
    println!("{}", mensaje);
    //println!("{}", mensaje_dos); esto me da error por que no estan en el mismo scope 

    //OPTIMIZE como puedo exportar las variables de dichos bloques, lo recomendables es asignar el bloque a una variable

    let resultado = {
        println!("HOLA NOS ENCONTRAMOS EN OTRO BLOQUE ANIDADO");

        let numero: i32 = 200;

        println!("{}", numero);

        numero  //TODO de esta forma se retorna el valor de un bloque nos sirve cando trabajamos con funciones ciclos ,condiciones y match
    };

    println!("el valor que nos devuelve el bloque es al main: {}", resultado);

    let calificacion: i8 = 10;

    //TODO: metodo uno asignando una variable
    //let mut mensaje_estudiante = String::new();
    //TODO: metodo 2 es devolver directamente la respuesta de la condicion a una variable

    let mensaje_estudiante: String = if calificacion == 10 {
        //mensaje_estudiante = String::from("felicitaciones");//cuando reaccicnamos el valor lleva(;)
        String::from("felicitaciones")//TODO cuendo devolvemos un valor sin asignar a una variable no lleva punto y coma (;)
    }else {
        //mensaje_estudiante = String::from("Necesitas estudiar mas");
        String::from("Necesitas estudiar mas")
    };//FIX importante caundo tomamos como bloque asignado a una variable cierra con (;) bloque
    
    println!("{}", mensaje_estudiante);
}
