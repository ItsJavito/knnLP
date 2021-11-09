use crate::classes::persona::Persona;
use crate::classes::utility::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DataFrame{
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

    pub fn calc_distance(&mut self , persona: &Persona) -> Vec<(f32 , &String)>{

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
            
            sum += (((persona.peso - peso_min) / (peso_max - peso_min)) - 
            ((self.peso[i] - peso_min) / (peso_max - peso_min))).powf(2.0); 
            
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
            sum += (trans_data(persona.mtran.to_string()) - 
            trans_data(self.mtran[i].to_string())).powf(2.0);
            //----------------------------------------------
            /*
            
            valores de las distancias para cada caso 
            
            NO | Sometimes | Frequently | Always
            0  |    0.33   |     0.66   |  0.99 
            */
            
            // no , sometimes , frequently, always
            sum += (categoric_data(persona.caec.to_string()) - 
            categoric_data(self.caec[i].to_string())).powf(2.0);
            
            // no , sometimes, frequently , always 
            sum += (categoric_data(persona.calc.to_string()) - 
            categoric_data(self.calc[i].to_string())).powf(2.0);

            sum = sum.sqrt();

            dist.push((sum, obesidad));
        }
        dist.sort_by(|a, b| a.partial_cmp(b).unwrap());
        dist
    }
    pub fn knn( &mut self , mut k: i64 , persona: &Persona ) -> String {
        let ar = self.calc_distance(persona);
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

}