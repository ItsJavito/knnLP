extern crate csv;
mod classes;
use crate::classes::persona::Persona;
use crate::classes::data_frame::DataFrame;
use std::time::Instant;


/**
 * Grupo
 * Javier Olazábal 
 * Sebastian Chávarry 
 * Fiorella Valdivia 
 * Frabrizio figari
*/

fn main() {
    //se lee la data desde el csv
    let mut data = DataFrame::read_csv("./data.csv", true);
    //creamos la persona y pedimos por consola los datos
    let mut persona = Persona{
        gender : String::from("Female"),
        age : 15.0,
        altura : 1.7,
        peso : 80.0, 
        family_overweight : String::from("yes"),
        favc: String::from("yes"),
        fcvc: 2.0, 
        ncp: 2.0,
        caec: String::from("Always"),
        smoke: String::from("no"),
        ch20: 3.0,
        scc: String::from("yes"),
        faf: 0.0,
        tue: 0.0,
        calc: String::from("no"),
        mtran: String::from("Walking"),
        nobey : String::from("")
    };

    // asignamos un k 
    let k = 30;    
    //hacemos el knn que nos dará como resultado el tipo de obesidad
    let start = Instant::now();
    persona.nobey = data.knn(k, &persona);
    let elapsed = start.elapsed();
    // ejecutar con cargo run --release
    println!("Tiempo de ejecucion: {:?}" , elapsed);
    println!("La clasificación de la persona es: {:?}", persona.nobey); 
}