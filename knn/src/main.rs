extern crate csv;
mod classes;

use crate::classes::persona::Persona;
use crate::classes::data_frame::DataFrame;

fn main() {
    //se lee la data desde el csv
    let mut data = DataFrame::read_csv("./data.csv", true);
    //creamos la persona y pedimos por consola los datos
    let mut persona = Persona::new();
    // asignamos un k 
    let k = 30;

    //creamos el vector de las distancias con el tiepo 
    //let dist : Vec<(f32, &String)> = data.calc_distance(&persona);
    
    //hacemos el knn que nos dará como resultado el tipo de obesidad
    persona.nobey = data.knn(30, &persona);

    println!("La clasificación de la persona es: {:?}", persona.nobey); 
}