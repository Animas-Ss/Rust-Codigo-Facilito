#[derive(Debug)]
struct User {
    name: String,
    password: String,
    email: String,
    age: Option<u32>,//opcional puede o no almacenar valor
}

//TODO OPTIONS es un enum con  dos valores si existe o no el valor
/*
  enum Option<T>{
     Some(T), -> el valor 
     None -> la ausencia de algun valor
  }

 */

/* fn obtener_valor(bandera: bool) -> Option<String>{
    if bandera {
        Some(String::from("soy un mensaje para la tupla some!"))
    }else{
        None
    }
} */

//TODO: option en una estructura


fn main() {
    //Option -> Si existe o no algun valor
    //Result -> Errors -> panic!


    //let resultado = obtener_valor(false); // Option

    //unwrap -> obtiene lo que la tupla Some almacene
    //unwrap_or -> podemos colocar una opcion para el none
    //expect -> nos deja optener el valor de Some , si no nos deja poner lo que se esperaba resibir
    //let valor = resultado.expect("Se esperaba un String");// resive el mismo dato que resive la Option<String> o el que se declare
    //let valor = resultado.unwrap_or("Sebastian no tiene datos".to_string());// resive el mismo dato que resive la Option<String> o el que se declare
    //let valor = resultado.unwrap();
    //println!("el valor: {}", valor);
   /*  match resultado {
        Some(valor) => println!("El valor es: {}", valor),
        None => println!("No existe valr alguno")
    }; */

    //TODO: como implementar Opcion en una structura segundo ejemplo
    let usuario_uno = User{
        name: String::from("Sebastian"),
        password: String::from("123456"),
        email: String::from("seba_sosa@gmail.com"),
        age: Some(26),
    };

    println!("El usuario es: {:?}", usuario_uno);
}
