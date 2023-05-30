fn main() {
    //TODO : TIPO DE DATOS

    //OPTIMIZE : ENTEROS
    //i8, i16, i32, i64, i128 -> con el prefijo i suena como imaginarios , lo que referimos que peuden tener signo
    //u8, u16, u32, u64, u128 -> con el prefijo u suena como reales , solo positivos

    let numero_uno: i8 = -10;
    let nuemro_dos: u8 = 10;

    println!("{} {}", numero_uno, nuemro_dos);
    //OPTIMIZE : CHAR
    // char -> los representamos con comillas simples el ancho es de 32 bit
    //Rust utiliza el codificado UTF-8 podemos usar emoji
    let caracter: char = '🦀';

    println!("caracter : {}", caracter);

    //OPTIMIZE : FLOAT
    //f32, f64 -> en el caso de los flotantes pueden decidir como en el resto cual es el tipo mas conveniente y a diferencia de los enteros ya pueden usar el signo
    let real: f32 = 12.5;
    let real_netativo: f32 = -12.5;

    println!("numeros flotantes ({})- ({})", real, real_netativo);

    //OPTIMIZE: BOOLEANOS
    // true, false -> los booleanos son de 8bit y pueden tener el valor  true o false
    let llueve: bool = false;
    let funciona: bool = true;

    println!("llueve: {}, funciona: {}", llueve, funciona);

    //OPTIMIZE STR , STRING

}
