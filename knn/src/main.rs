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
    //hacemos el knn que nos dará como resultado el tipo de obesidad
    persona.nobey = data.knn(k, &persona);
    println!("La clasificación de la persona es: {:?}", persona.nobey); 
}