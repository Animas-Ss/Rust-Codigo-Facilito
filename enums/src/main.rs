fn main() {
    //TODO Enum junto con el match son los fiuchurs mas poderosos 
    //Enum es un typo que almacena diferentes variantes 

    enum Response {
        Success,
        Error(u32, String),//los elementos pueden tener codigo en este caso es una tupla de un solo elemento , tomamos los elementos nuemricos u32 ya que los errores pueden ser 403, 404, 500
    }

    let respuesta: Response = Response::Error(501, String::from("no es posible completar la operacion"));

    match respuesta {
        Response::Success => println!("La peticion se realizo exitosamente"),
        Response::Error(403, _) => println!("Forbidden") ,
        Response::Error(404, _) => println!("Not Found") ,
        Response::Error(500, _) => println!("Internal server Error"),
        Response::Error(_, mensaje) => println!("{}",mensaje),
    }
}
