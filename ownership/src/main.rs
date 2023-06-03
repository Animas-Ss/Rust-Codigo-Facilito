//TODO para entender este concepto vamos a declarar una structura 
struct Rectangulo {
    ancho: u32,
    alto: u32
}
//TODO: declaramos una funcion que saque el area de nuestro rectangulo
//caundo trabajamos con un valor prestado y queremos devolverlo tenemos que colocar en nuestro argumento 
//dicha orden en Rust esto se indica con el & de que es una referencia un valor prestado una evz que se use se devuleve a su propietario
fn area (rectangulo: &Rectangulo)-> u32{
    rectangulo.ancho * rectangulo.alto// calculamos el area y devolvemos el resultado
}

fn main() {
    //ownership es el recolector de basura de rust , el que se acegura de la buena utilizaciond e la memoria
    //TODO: EL OWNERSHIP TIENE 3 REGLAS
    //TODO: 1 cada valor en Rust tiene su propio ownership
    //TODO: 2 solo puede existir un ownership a la vez
    //TODO: 3 si el ownership sale del alcanse su valor se descartara 

    //FIX esto ahce referencia a los prestamos de valores , alcances

    let rectangulo_uno = Rectangulo{ ancho: 10, alto:20};// creamos nuestro objeto con las medidas asigandas
    //let resultado = area(rectangulo_uno);//calculamos el area de nuestro primer objeto en esta linea estamos cambiando de scope a la variable rectangulo dejando asi este bloque
    //TODO: es importante en esta parte recordar que los argumentos son pasados como prestamos a las funciones o metodos 
    //TODO: por lo que una vez que los prestamos dejan de existir en el bloque y pasan a otro bloque en este caso el bloque area
    //TODO: una vez termina el bloque area estas variables son destruidas y al dejar de exitir en el calcance de mi main no puedo vlverlas a usar
    //fix ownership se refiere a pripietario solo puede  tener un propietario , este comportameinto descripto es por default en rust
    //OPTIMIZE: solucion !! pasar los valores por referencias , de esta manera gestionamos mejor el usu de momeria como sugiere Rust
    //OPTIMIZE la solucion a esto es colocar el & delante de la variable prestada indicando a Rust que queremos el valor devuelta una ves sea usado en el otro ambito o bloque
     
     let resultado = area(&rectangulo_uno);

     //TODO: importante a tener en cuenta que sucede si prestamos el ownership
     //FIX esto de dejar inservible las variables prestadas o tomadas por neuvas solo aplica para las variables almacenadas en el HEAP
     //FIX ya que son variables que estan en costante movimeitno , de tamaño creciendo o decreciendo o modificando su valor como estos objetos
     let nuevo_rectangulo = rectangulo_uno;// estamos prestando el ownership por lo cual deja de exitir rectangulo_uno y pasa a ser el dueño nuevo rectangulo

     //TODO: en cambio para los valores almacenados en el stack esto no pasa por ejemplo no dejan de existir si asignamos su valor
     let x = 10;
     let y = x;
     // como estan almacenadas en el stack no hace falta concoer su ownership y no se destruyen
     println!("x: {}", x);
     println!("y: {}", y);

    println!("el area es: {}", resultado);
    //para usar la linea siguiente recordar que los valores tienen que ser pasados como referencias prestados y devueltos asi no se pierde el alcance a los mismo
    //println!("el ancho y el alto del triangulo es : {} - {}", rectangulo_uno.ancho, rectangulo_uno.alto); //deja de funcionar por que rectangulo dejo de exitir a reacicnar su valor a neuvo_rectangulo
    println!("el ancho y el alto del triangulo es : {} - {}", nuevo_rectangulo.ancho, nuevo_rectangulo.alto);
}
