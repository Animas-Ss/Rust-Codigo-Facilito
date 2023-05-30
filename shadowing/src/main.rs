fn main() {
    //OPTIMIZE este concepto nos hace referencia a poder usar el mismo nombre de una variable y reasignar el valor
    //fix tener en cuenta que caundo usamos esto el valor antenior es destruido y el nombre pasa a tener un nuevo valor y tambien tipo de necesitarlo
    //definimos una variable
    let value: i32 = 15;// por defecto inmutable ;
    println!("el valor de la variable es: {}", value);
    let value: i8 = 6; //shadowing
    println!("el valor de la variable es: {}", value);
    let value: bool = false;
    println!("el valor de la variable es: {}", value);

    //TODO: todas las variables siguen siendo inmutables , lo que sucede antes de la asignacion es la eliminacion de la anterior 
}
