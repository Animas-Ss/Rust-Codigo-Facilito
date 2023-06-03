fn main() {
    let numero: i32 = 50;

    match numero {
        //todo valor => sentencia / tarea
        1 => println!("el nuemero es uno."),
        2 => println!("el numero es dos."),
        3 => println!("el numero es tres."),
        4 | 5 | 6 => println!("el numero se encuentra entre 4 y 6"),//TODO condiciones! o valores diferentes 
        7..=100 => {//TODO un bloque de tareas par realizar multiples tareas
            println!("el numero es mayor o igual a 7");
            println!("el numero se evalua mediante un rango");
            if numero % 7 == 0 {
                println!("numero divisible en 7");
            }else{
                println!("el numero no es divicible en 7");
            }
        },
        _ => println!("el numero no esta contemplado en estos casos, su numero fue: {}", numero) //SE USA PARA DEFAULT
    }

    //OPTIMIZE otra forma de usar el Match_ y los bloques es con la asignacion de una variable al match asi nos devuelve los valores

    let mensaje: &str = match numero {
        //todo valor => sentencia / tarea
        1 => "el nuemero es uno.",
        2 => "el numero es dos.",
        3 => "el numero es tres.",
        4 | 5 | 6 => "el numero se encuentra entre 4 y 6",//TODO condiciones! o valores diferentes 
        7..=100 => {
                let mensaje = "el numero es evaluado mediante un rango del 7 al 100";
                mensaje
        },
        _ => "numero"
    };
    println!("Programa de swhict, y el resultado es  {}", mensaje);

    //OPTIMIZE MATCH dos usando fizz buzz
    //TODO creamos un ciclo for con un rango de 1 al 30 dentro colocamos nuestro match
    for numero in 1..31 {
        //TODO en la siguiente linea evaluamos dos casos por esoe sta en una tupla
        match (numero % 3, numero % 5) {
            (0, 0) => println!("Fizz Buzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),//FIX el _ guion bajo representa el valor que obtenemos osea cualqueira lo que representa al condicion a cumplr de la tupla
            _ => println!("{}", numero)// si ninguno cumple esta condicion imprimimos el nuemro (_, _) => otra forma tabein seria considerar el default asi
        }
    }
//FIX a considerar match para los proyectos ya que es muy util por que se peuden evaluar objetos Result, arreglos, etc;
}
