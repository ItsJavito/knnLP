extern crate csv;
extern crate cpu_monitor;
mod classes;
use crate::classes::data_frame::DataFrame;
use crate::classes::persona::Persona;
use std::time::Instant;
extern crate mem_macros;
use std::io;
use std::time::Duration;
use cpu_monitor::CpuInstant;

/**
 * Grupo
 * Javier Olazábal
 * Sebastian Chávarry
 * Fiorella Valdivia
 * Fabrizio figari
*/

fn main() -> Result<() , io::Error> {
    let mut memoria:f32;
    memoria = 0.0;  
    //se lee la data desde el csv
    let mut data = DataFrame::read_csv("./data.csv", true);
    //creamos la persona y pedimos por consola los datos
    let mut persona = Persona {
        gender: String::from("Female"),
        age: 15.0,
        altura: 1.7,
        peso: 80.0,
        family_overweight: String::from("yes"),
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
        nobey: String::from(""),
    };
    let k = 30;
    //hacemos el knn que nos dará como resultado el tipo de obesidad
    let start = Instant::now();
    let startcpu = CpuInstant::now()?;
    persona.nobey = data.knn(k, &persona);
    let elapsed = start.elapsed();
    let endcpu  = CpuInstant::now()?;
    let durationcpu = endcpu - startcpu;
    // ejecutar con cargo run --release
    println!("Tiempo de ejecucion: {:?}", elapsed);
    println!("La clasificación de la persona es: {:?}", persona.nobey);
    ok(())
}
