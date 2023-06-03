fn main() {
    //TODO: el str -> Stack - es inmutable y se conoce su medida de antemano 
    //TODO: String -> Heap - es mutable y puede varian en tiempod e ejecucion

    //OPTIMIZE str
    let variable_str = "Hola soy una variable str";
    let variable_string = String::new();

    //TODO: podemos generar un str desde un string mediente el metodo from()
    let str_desde_string = String::from("soy un str, desde un string");

    //TODO: ahora le sacamos todo el provecho combiertiendo nuestro string en mutable
    let mut string_mutable = String::from("Hola, ahora voy a agregar");

    string_mutable.push(' ');
    string_mutable.push('A');
    string_mutable.push('N');
    string_mutable.push('I');
    string_mutable.push('M');
    string_mutable.push('A');
    string_mutable.push('S');
    //TODO: agregamos un str al final del string
    string_mutable.push_str(" Es el mejor nick del mundo");

    //TODO: convertimos un tipo str a un string
    let conversion_str = "No olvides lo mas importante, comprender";
    conversion_str.to_string();

    //TODO: con los string podemos usar comparadores logicos asi comparamos cadenas

    let iguales = conversion_str == "No olvides lo mas importante, comprender.".to_string();



    //TODO: ejecutamos las variables 
    println!("esta es mi variable str: {}", variable_str);
    println!("esta es mi variable string {}", variable_string);
    println!("generamos un str desde un string : {}", str_desde_string);
    println!("Modificando el string: {}", string_mutable);
    println!("son iguales las cadenas de string: {}", iguales);
}
