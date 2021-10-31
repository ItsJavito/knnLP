extern crate csv;
use std::collections::HashMap;
use std::*; 

//------------------------ DATA FRAME -----------------------------
#[derive(Debug)]
struct DataFrame {
   header: csv::StringRecord,
   gender: Vec<String>,
   age:  Vec<f32>,
   altura: Vec<f32>,
   peso: Vec<f32>,
   family_overweight : Vec<String>,
   favc: Vec<String>,
   fcvc: Vec<f32>,
   ncp: Vec<f32>,
   caec: Vec<String>,
   smoke: Vec<String>,
   ch20: Vec<f32>,
   scc: Vec<String>,
   faf: Vec<f32>,
   tue: Vec<f32>,
   calc: Vec<String>,
   mtran: Vec<String>,
   nobey: Vec<String>
 }

 struct Persona{
    gender: String,
    age:  f32,
    altura: f32,
    peso: f32,
    family_overweight : String,
    favc: String,
    fcvc: f32,
    ncp: f32,
    caec: String,
    smoke: String,
    ch20: f32,
    scc: String,
    faf: f32,
    tue: f32,
    calc: String,
    mtran: String,
    nobey: String 
 }

impl DataFrame {

    fn new() -> DataFrame {
        DataFrame {
            header: csv::StringRecord::new(),
            gender: Vec::new(),
            age: Vec::new(),
            altura: Vec::new(),
            peso: Vec::new(),
            family_overweight : Vec::new(),
            favc: Vec::new(),
            fcvc: Vec::new(),
            ncp: Vec::new(),
            caec: Vec::new(),
            smoke: Vec::new(),
            ch20: Vec::new(),
            scc: Vec::new(),
            faf: Vec::new(),
            tue: Vec::new(),
            calc: Vec::new(),
            mtran: Vec::new(),
            nobey: Vec::new()
        }
    }

    pub fn read_csv(filepath: &str, has_headers: bool) -> DataFrame {
        // Open file
        let file = std::fs::File::open(filepath).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .from_reader(file);

        let mut data_frame = DataFrame::new();

        // push all the records
        for result in rdr.records().into_iter() {
        let record = result.unwrap();
        data_frame.push(&record);
        }
        return data_frame;
    }

    fn push(&mut self, row: &csv::StringRecord) {
        self.gender.push(row[0].to_string());
        self.age.push(row[1].parse().unwrap());
        self.altura.push(row[2].parse().unwrap());
        self.peso.push(row[3].parse().unwrap());
        self.family_overweight.push(row[4].to_string());
        self.favc.push(row[5].to_string());
        self.fcvc.push(row[6].parse().unwrap());
        self.ncp.push(row[7].parse().unwrap());
        self.caec.push(row[8].to_string());
        self.smoke.push(row[9].to_string());
        self.ch20.push(row[10].parse().unwrap());
        self.scc.push(row[11].to_string());
        self.faf.push(row[12].parse().unwrap());
        self.tue.push(row[13].parse().unwrap());
        self.calc.push(row[14].to_string());
        self.mtran.push(row[15].to_string());
        self.nobey.push(row[16].to_string());
    }

    fn calc_distance(&mut self , persona: &Persona) -> Vec<(f32 , &String)>{

        let mut dist : Vec <(f32, &String)> = Vec::new();
        let (age_min,  age_max) = get_min_max(&self.age);
        let (alt_min,  alt_max) = get_min_max(&self.altura);
        let (peso_min,  peso_max) = get_min_max(&self.peso);

        let (fcvc_min,  fcvc_max) = get_min_max(&self.fcvc);
        let (faf_min,  faf_max) = get_min_max(&self.faf);
        let (ch20_min, ch20_max) = get_min_max(&self.ch20);
        let (ncp_min,  ncp_max) = get_min_max(&self.ncp);
        let (tue_min,  tue_max) = get_min_max(&self.tue);

        for i in 1..self.gender.len(){
            let mut sum : f32 = 0.0;
            let obesidad : &String = &self.nobey[i];

            sum += ((persona.age - age_min) / (age_max - age_min) - 
            (self.age[i] - age_min) / (age_max - age_min)).powf(2.0);
            
            sum += ((persona.altura - alt_min) /(alt_max - alt_min) - 
            (self.altura[i] - alt_min) / (alt_max - alt_min)).powf(2.0);
            
            sum += ((persona.peso - peso_min) / (peso_max - peso_min) - 
            (self.peso[i] - peso_min) / (peso_max - peso_min)).powf(2.0); 
            
            sum += ((persona.fcvc - fcvc_min) / (fcvc_max - fcvc_min) - 
            (self.fcvc[i] - fcvc_min) / (fcvc_max - fcvc_min)).powf(2.0); 

            sum += ((persona.faf - faf_min) / (faf_max - faf_min) - 
            (self.faf[i] - faf_min) / (faf_max - faf_min)).powf(2.0); 

            sum += ((persona.ch20 - ch20_min) / (ch20_max - ch20_min) - 
            (self.ch20[i] - ch20_min) / (ch20_max - ch20_min)).powf(2.0); 

            sum += ((persona.ncp - ncp_min) / (ncp_max - ncp_min) - 
            (self.ncp[i] - ncp_min) / (ncp_max - ncp_min)).powf(2.0); 

            sum += ((persona.tue - tue_min) / (tue_max - tue_min) - 
            (self.tue[i] - tue_min) / (tue_max - tue_min)).powf(2.0); 

            // male or female
            if persona.gender != self.gender[i] {sum += 1.0;}

            // yes or no
            if persona.family_overweight != self.family_overweight[i] {sum += 1.0;}
            
            //yes or no
            if persona.favc != self.favc[i] {sum += 1.0;} 
            
            // yes or no 
            if persona.smoke != self.smoke[i] {sum += 1.0; }

            // yes or no
            if persona.scc != self.scc[i] {sum += 1.0;}
            
            // Automobile, Motorbike , Bike, Public_Transportation , Walking
            if persona.mtran != self.mtran[i] {sum += 1.0; }


            //----------------------------------------------
            /*
            Poner los valores para cada tipo de respuesta. 
            FALTA IMPLEMENTAR 

            NO | Sometimes | Frequently | Always
            0  |    0.33   |     0.66   |  0.99 
            */

            // no , sometimes , frequently, always
            if persona.caec != self.caec[i] {sum += 1.0;}

            // no , sometimes, frequently , always 
            if persona.calc != self.calc[i] {sum += 1.0;}



            sum = sum.sqrt();

            dist.push((sum, obesidad));
        }
        dist.sort_by(|a, b| a.partial_cmp(b).unwrap());
        dist
    }

}

//---- FUNCIONES ----

//Se esta implementando por que en std no hay implementacion con f32
// pasamos vectores por referencia y obtenemos el valor maximo del vector
fn get_max(ar :&Vec<f32>) ->  f32{
    let mut max = f32::MIN; 
    for &i in ar{
        if i > max { max = i;}
    }
    max
}

fn get_min(ar : &Vec<f32>) -> f32{
    let mut min = f32::MAX;
    for &i in ar{
        if i < min { min = i;} 
    }
    min
}

fn get_min_max (ar : &Vec<f32>) -> (f32, f32){
    let min = get_min(ar);
    let max = get_max(ar);
    (min, max)
}

fn knn(mut neigh: i64 , ar : &Vec<(f32 , &String)> ) -> String {
    let mut res : String = String::from("");
    let mut nearest : HashMap<&String , i32> = HashMap::new();
    let mut indx = 0;
    
    while neigh > 0 
    {
        let (_val, tipo) = &ar[indx]; 
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


    /*
    
        COSAS QUE FALTAN IMPLEMENTAR 
        1. INPUT DE PERSONA , CON LAS PREGUNTAS DE CADA <-- LEER PDF DE EVALUACIONES
        2. PONER LOS VALORES PARA LOS DATOS CATEGÓRICOS
        3. CONSIDERAR CALCULAR EL IMC Y PONERLO COMO OTRO ATRIBUTO
    
    */
    
    let mut persona = Persona{
        gender : String::from("Male"),
        age : 18.0,
        altura : 1.87,
        peso : 173.0, 
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

    let dist : Vec<(f32, &String)> = data.calc_distance(&persona);
    persona.nobey = knn(20, &dist);
    println!("{:?}" , dist);
    println!("{:?}", persona.nobey); 
}