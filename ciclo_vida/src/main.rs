fn main() {//BLOQUE 1 main

    //TODO: Bloque limita el scope de una variable 
    let mensaje= "Hola , soy una variable del bloque main";// alcance global
    println!("Bloque 1: {}", mensaje);

    //TODO: ciclo de vida con los prestamos 
    let mensaje_prestamo = String::from("Hola, soy una variable para prestamo");


    {
        //BLOQUE 2
        //TODO: el bloque dos ya no utiliza la variable mensaje del bloque uno ya que tiene una variable del mismo nombre en su scope
        let mensaje = "Hola , soy una variable del bloque dos";
        println!("Bloque 2: {}", mensaje);
        //FIX importante si en un bloque se usa una variable y la misma no esta en su scope , busca de forma asendente dicha variable 
        //FIX en este ejemplo bloque dos y tres usan la misma variable , bloque 3 por que toma al bloque 2 como padre entonces usa su variable
        //OPTIMIZE lineas de codigo para ejemplo de ciclo de vida con variables prestadas

        let _prestamo = &mensaje_prestamo;// asiciendo esto la variable se mueve y deja de existir en el main si no le ponemos el simbolod e prestamo &
        
        //mensaje_prestamo= String::from("cambio de valor");// no se peude modificar un valor perstado hasta que este deje de ser usado

        println!("{}", _prestamo);//freezing , la variable se congela hasta que se utiliza
        {
            //BLOQUE 3
            println!("Bloque 3: {}", mensaje);
            //TODO como se aclaro antes si declaro una variable en este bloque no va a poder ser usada en los padres ya que se destruye una vez el bloque termine
            let _resultado = 10 + 20;// a esto se lo conoce como alcance

        }
    }
    println!("{}", mensaje_prestamo);
}
