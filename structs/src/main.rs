struct User {
    name: String,
    password: String,
}
//TODO: vamos a crear un constructor que cumpliria la funcion de crear nuevos usuarios con una funcion

fn create_user(name: String , password: String) -> User {
   return  User { name, password};// User{name, password} sin ;
}

fn main() {
    //TODO structs son como declarar typos o constructores de claces , lo podria relacionar con eso 

    let usuario = User {
        name: String::from("Animas"),
        password: String::from("animas123"),
    };

    //TODO: podemos declarar nuestro objeto con las variables por afuera

    let name = String::from("Animas_dos");
    let password = String::from("animas123_dos");

    let usuario_dos = create_user(name, password);//Creamos un usuario neuvo llamando a nuestra funcion para crear usuarios con los valores requeridos que los tenemos declarados ariba pero podemos solicitarselo al usuario
    //TODO: para cambiar los valores de mi objeto tengoq ue hacerlo mutable 

    let name_tres = String::from("animas_tres");
    let password_tres = String::from("123456");

    let mut usuario_tres = create_user(name_tres, password_tres);//al hacerlo mutable cambiamos los valores

    usuario_tres.name = String::from("Cambio de valor");//asi pasamos el nuevo valor en nuestro objeto usuarios

    println!("Usuario: {}", usuario.name);
    println!("Password: {}", usuario.password);
    println!("Usuario dos : {}", usuario_dos.name);
    println!("Usuario dos : {}", usuario_dos.password);
    println!("usuario tres: {}", usuario_tres.name);
    println!("usuario tres: {}", usuario_tres.password);

}
