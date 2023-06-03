#[derive(Debug)]//nos permite analizar la structura con el {:?} es un decorador que le indica a esta estructura esa orden
struct User {
    name: String,
    password: String
}
//TODO: como creamos un objeto con sus metodos
//OPTIMIZE: aca es caundo utilizamos como en otros lenguajes el self que para java o javasCript es el this
impl User {
    //le damos permisos mutables a nuestra estructura
    fn saludar(&mut self){
        println!("hola Usuario: {}", self.name);
    }
    //TODO: creamos un metodo un poco mas complejo cambio de contraseña
    fn change_password(&mut self, new_password: String){
        self.password = new_password;//le pongo ; por que no queiro que retorne el neuvo valor
    }
}
fn main() {
    //TODO metodos , vamos a realizar un ejemplo de esto con la structura User
    
    let mut usuario_uno = User{
        name: String::from("Sebastian"),
        password: String::from("animas123")
    };

    usuario_uno.saludar();// acabamos de crear un metodo de nuestra structura Usuario
    
    usuario_uno.change_password("perro gato".to_string());//recordar que esto es un str por lo caul tene mos que convertirlo en un string con .to_string
    println!("el usuario es: {:?}", usuario_uno);
}
