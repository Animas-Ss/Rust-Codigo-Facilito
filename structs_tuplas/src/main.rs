//TODO: estructuras de tipo tuplas , en vez de poseer atributos , posee valores
#[derive(Debug)]
struct Color (u32, u32, u32); //OPTIMIZE los colores se componen de tres tonalidades RGB

fn main() {
    //DEFINIMOS los colores

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    //TODO: podemos tratar los elementos como una tupla
    let mut custome_color = Color(187, 62, 104);

    custome_color.1 = custome_color.1 + 10;//quedaria 72

    println!("El color es: {:?}", black);
    println!("El color es: {:?}", white);
    println!("cambiando el color variable de la tupla: {:?}", custome_color);
}
