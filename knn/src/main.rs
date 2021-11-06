extern crate csv;
mod classes;

use crate::classes::persona::Persona;
use crate::classes::data_frame::DataFrame;

use std::collections::HashMap;
use std::*; 


//Algoritmo KNN 

fn knn(mut neigh: i64 , ar : &Vec<(f32 , &String)> ) -> String {

    let mut res : String = String::from("");
    let mut nearest : HashMap<&String , i32> = HashMap::new();
    let mut indx = 0;
    
    while neigh > 0 
    {
        let (_, tipo) = &ar[indx]; 
        *nearest.entry(tipo).or_insert(0) += 1;
        indx += 1; neigh -= 1;
        //si ya no faltan mas vecinos que recorrer
        if neigh == 0{
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
                neigh += 1;
            }
        }
    }
    res 
}


fn main() {
    let mut data = DataFrame::read_csv("./data.csv", true);
    let mut persona = Persona::new();
    
    let dist : Vec<(f32, &String)> = data.calc_distance(&persona);
    persona.nobey = knn(30, &dist);
    
    println!("La clasificación de la persona es: {:?}", persona.nobey); 
}