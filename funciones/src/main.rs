//TODO: estructuraa basica de una funcion fn <nombre> en snake case () argumentos o parametros -> returno lo que nos devuelve
fn suma(numero_uno: i32, numero_dos: i32) -> i32 {
    numero_uno + numero_dos // caundo el return es inplicito no se escbribe y no lleva ;
}

//OPTIMIZE: funcion para calcular el factorial de un numero 
//FIX concepto importante recurcion donde dentro de una funcion llamamos a la misma 
//el factorial es la multiplicacion de todos los numeros enteros positivos o iguales anteriores al numero ingresado
fn factorial(n: u32) -> u32 {
    if n == 1 {
        n //return n;
    }else {
        factorial(n - 1) * n// a esto e le denomina recursion donde llamamos a la funcion reducimos su numero en 1 y multiplicamos el resultado por el numero esto nos devuelve la multiplicacion del numero por su anterior 
    }
}

fn main() {
    //TODO: funciones , estructura de una funcion y como usarla
    let response = suma(20, 30);// aca deberia de guardar el resultado en una variable para poderlo ver
    println!("suma: {}", suma(10, 20));// puedo pasar una funcion de retorno en el print para poder ver su retirno
    println!("response de la suma : {}", response);

    println!("el factorial es : {}", factorial(5));
}
