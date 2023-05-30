fn main() {

    let numero_uno: i32 = 13;
    let numero_dos: i32 = 200;

    //OPTIMIZE : operadores (+) suma
    let resultado: i32 = numero_uno + numero_dos;
    println!("resultado: {}", resultado);
    //OPTIMIZE : operadores (-) resta
    let resultado: i32 = numero_uno - numero_dos;
    println!("resultado: {}", resultado);
    //OPTIMIZE : operadores (/) division 
    let resultado: i32 = numero_dos / numero_uno;
    println!("resultado: {}", resultado);
    //OPTIMIZE : operadores (*) multiplicacion
    let resultado: i32 = numero_uno * numero_dos;
    println!("resultado: {}", resultado);
    //OPTIMIZE : operadores (%) resto de la division
    let resultado: i32 = numero_dos % numero_uno;
    println!("resultado: {}", resultado);

    //TODO : operadores relacionales (>) mayor
    let resultado: bool = numero_uno > 100;
    println!("resultado: {}", resultado);
    //TODO : operadores relacionales (<) menor
    let resultado: bool = numero_uno < 100;
    println!("resultado: {}", resultado);
    //TODO : operadores relacionales (>=) mayor igual
    let resultado: bool = numero_uno >= 100;
    println!("resultado: {}", resultado);
    //TODO : operadores relacionales (<=)menor igual
    let resultado: bool = numero_uno <= 100;
    println!("resultado: {}", resultado);
    //TODO : operadores relacionales (==) igual
    let resultado: bool = numero_uno == 100;
    println!("resultado: {}", resultado);
    //TODO : operadores relacionales (!=) desigual
    let resultado: bool = numero_uno != 100;
    println!("resultado: {}", resultado);

    //OPTIMIZE: operadores logicos (&&) and tienen que cumplirse ambas condiciones para que el resultados ea true sino es false
    let resultado: bool = 10 + 20 == 30 && numero_dos < 300;
    println!("resultado condicion and: {}", resultado);
    //OPTIMIZE: operadores logicos (||) or tiene que cumplirse una de las dos para que el resultado sea true ambos tiene que ser false par que el resultado sea false
    let resultado: bool = numero_uno == 200 || numero_uno == 13;
    println!("resultado condicion or: {}", resultado);


}
