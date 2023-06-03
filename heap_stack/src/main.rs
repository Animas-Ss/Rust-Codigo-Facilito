fn main() {
    //OPTIMIZE : esta son astracciones que nos permiten entender en que lugar de la memoria se almacenan nuestras variables 
    //TODO: HEAP
    /*//FIX en el Heap encontramos todas las variables donde su tamaño pueda variar en el tiempo de ejecucion como los string o los vectores
       como desconocemos el tamaño que puedan ocupar ya que pueden cambiar en tiempo de ejecucion se almacenan en el heap
       este presenta una disminucion en la velocidad de escritura o lectura 
       a comparacion del stack 
     */
    //TODO: Stack
    /*
    en esta astraccion se encuentran alojadas todas las variables
    donde de antemano ya conocemos su longitud por ejemplo
    let mut x: i32 = 10; 
    en este caso se almacena en el stack ya que conocemos su logitud que es de 32 bit
     //FIX el Stack tiene la particularidad de ser muy rapido lo vemos comouna pila
     en este caso un lifo o una pila
     fn foo(){
        let b = 10;
        let c = 20;
     }

     fn main(){
        let a = 30;
        foo();
     }

     si consideramos que las variables dentro de la funcion foo se declaran despues que la variable a y cumplimos 
     el concepto de pila o lifo quedarian arriba de dicha pila o lifo por lo cual cuando se termine la ejecucion de 
     las mismas y sean destruidas , va a ser mas facil eliminarlas al estar al comienzo de la pila asi evitamos recalcular el tamaño de la pila
     */
    println!("astracciones Heap Stack");
}
