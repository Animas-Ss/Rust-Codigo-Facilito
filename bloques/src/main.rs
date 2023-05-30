fn main() {
    //TODO: EJEMPLO DE VIDA DE LAS VARIABLES O CODIGO EN UN BLOQUE 

    let mensaje: &str = "Hola, soy una vaiable en el bloque main.";
    {

        println!("Hola soy un segundo bloque");

    }
    println!("{}", mensaje);
}
