//OPTIMIZE importamos la libreria de entradas y salidas input
use std::io;

fn main() {
    //TODO nos apoyamos con el macro println!
    println!("Ingresa el nombre de usuario:");
    //TODO: en el siguiente caso creamos una variable mutable
    //podemos pensar o comparar el String::new() como si fuera una clace y new un metodo de la clace
    //donde deducimos que new es un metodo static -> no neesita ser instancia para su utilizacion
    let mut username = String::new(); //FIX este metodo nos retorna un string vacio ""
    // el siguiente metodo resive un prestamos de una varile creada previamente
    // podemos decir que prestamos el valor de username usando el signo & , tomamos el mut como premiso de escritura , al prestar el valor
    // ya le damos permiso de lectura sobre nuestra variable le otorgamos premiso de escritura poniendo el puntero mutable
    // esta funcion read_line devuelve un Result el cual tiene un valor de exit o error
    io::stdin().read_line(&mut username);//TODO esto nos devuelve un elemento Result<ok, error>
    //TODO ahora generamos una nueva variable para que no tengamos un salto de linea, el salto de linea se da por el entter despues de asignar el valor
    let username = username.trim();//trim es un metodo que nos permite desacernos de estos saltos
    println!("el valor de la variable es: {}", username);
    println!("Ingrese la edad del usuario:");
    let mut age = String::new();
    io::stdin().read_line(&mut age);
    let age = age.trim();
    //TODO: al aprecer es un objeto de typo Result
    let age: i32 = age.parse().unwrap(); // el metodo parse nos devuelve un Result<ok, erro> unwrap nos da la respuesta ok
    println!("el usuario {} tiene la edad {}", username, age);
    //FIX IMPORTANTE ACLARAR QUE LA FUNCION read_line() resive cmo argumento un tipo string por eso las variables creadas son String::new()
}
