extern crate csv;
mod classes;

use crate::classes::persona::Persona;
use crate::classes::data_frame::DataFrame;

use std::collections::HashMap;


//Algoritmo KNN 
fn knn(mut k: i64 , ar : &Vec<(f32 , &String)> ) -> String {

    let mut res : String = String::from("");
    let mut nearest : HashMap<&String , i32> = HashMap::new();
    let mut indx = 0;
    
    while k > 0 
    {
        let (_, tipo) = &ar[indx]; 
        *nearest.entry(tipo).or_insert(0) += 1;
        indx += 1; k -= 1;
        //si ya no faltan mas vecinos que recorrer
        if k == 0{
            let mut max = i32::MIN;
            let mut flag:bool = false; 
            //Buscamos la moda del tipo de obesidad
            for (&nom , &cant) in &nearest{
                if cant > max {
                    res = nom.to_string();
                    max = cant;
                    flag = false;
                }
                else if cant == max{
                    flag = true;
                }
            }
            //Si la moda se repite en 2 tipos de datos entonces 
            // aumentamos en 1 el k
            if flag == true{
                k += 1;
            }
        }
    }
    res 
}


fn main() {
    //se lee la data desde el csv
    let mut data = DataFrame::read_csv("./data.csv", true);
    //creamos la persona y pedimos por consola los datos

    let mut persona = Persona{
        gender : String::from("Male"),
        age : 18.0,
        altura : 1.87,
        peso : 90.0, 
        family_overweight : String::from("yes"),
        favc: String::from("yes"),
        fcvc: 3.0, 
        ncp: 3.0,
        caec: String::from("Frequently"),
        smoke: String::from("no"),
        ch20: 2.0,
        scc: String::from("no"),
        faf: 2.0,
        tue: 1.0,
        calc: String::from("Sometimes"),
        mtran: String::from("Public_Transportation"),
        nobey : String::from("")
    };

    // asignamos un k 
    let k = 30;

    //creamos el vector de las distancias con el tiepo 
    let dist : Vec<(f32, &String)> = data.calc_distance(&persona);
    
    //hacemos el knn que nos dará como resultado el tipo de obesidad
    persona.nobey = knn(k, &dist);
    println!("{:?}" , dist); 
    println!("La clasificación de la persona es: {:?}", persona.nobey); 
}