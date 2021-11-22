use std::io; 
pub struct Persona{
    pub gender: String,
    pub age:  f32,
    pub altura: f32,
    pub peso: f32,
    pub family_overweight : String,
    pub favc: String,
    pub fcvc: f32,
    pub ncp: f32,
    pub caec: String,
    pub smoke: String,
    pub ch20: f32,
    pub scc: String,
    pub faf: f32,
    pub tue: f32,
    pub calc: String,
    pub mtran: String,
    pub nobey: String 
}

impl Persona{
    pub fn new() -> Persona{
            
        let gender:String;
        let age:f32;
        let altura:f32;
        let peso:f32;
        let family_overweight:String;
        let favc:String;
        let fcvc:f32;
        let ncp:f32;
        let caec:String;
        let smoke:String;
        let ch20:f32;
        let scc:String;
        let faf:f32;
        let tue:f32;
        let calc:String;
        let mtran:String;
        println!("Por favor, introduzca los números según corresponda en cada pregunta");
    
        loop
        {
            let mut input = String::new(); 
            println!("¿Cual es su género?\nOPCIONES:\n(0) Masculino\n(1) Femenino");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            gender = match input.trim().parse().unwrap(){
                0 => String::from("Male"),
                1 => String::from("Female"),
                _ => {println!("Ingresa un numero dentro de los parámetros"); continue}
            };
            break; 
        };

        loop
        {
            let mut input = String::new(); 
            println!("¿Cual es su edad?: ");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            age = input.trim().parse().unwrap();
            break; 
        };

    
        loop
        {
            let mut input = String::new(); 
            println!("¿Cual es su altura en metros?: ");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            altura = input.trim().parse().unwrap();
            break; 
        };

        
        loop
        {
            let mut input = String::new(); 
            println!("¿Cual es su peso en kilogramos?: ");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            peso = input.trim().parse().unwrap();
            break; 
        };

        
        loop
        {
            let mut input = String::new(); 
            println!("¿En su familia alguien sufre o ha sufrido de sobrepeso?\nOPCIONES:\n(0) SI\n(1) NO");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            family_overweight = match input.trim().parse().unwrap(){
                0 => String::from("yes"),
                1 => String::from("no"),
                _ => {println!("Ingresa un numero dentro de los parámetros"); continue}
            };
            break; 
        };

        loop
        {
            let mut input = String::new(); 
            println!("¿Come comida alta en calorias frecuentemente?\nOPCIONES:\n(0) SI\n(1) NO");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            favc = match input.trim().parse().unwrap(){
                0 => String::from("yes"),
                1 => String::from("no"),
                _ => {println!("Ingresa un numero dentro de los parámetros"); continue}
            };
            break; 
        };
        
    

        loop
        {
            let mut input = String::new(); 
            println!("¿Come vegetales en su comida?\nOPCIONES:\n(0) Nunca\n(1) A veces\n(2) Siempre");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            fcvc = match input.trim().parse().unwrap(){
                0 => 1.0,
                1 => 2.0,
                2 => 3.0,
                _ => {println!("Ingresa un numero dentro de los parámetros"); continue}
            };
            break; 
        };
    
    

        loop
        {
            let mut input = String::new(); 
            println!("¿Cuatas comidas principales ingiere en el día?\nOPCIONES:\nIngrese un numero en el rango entre 1 - 4");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            ncp = match input.trim().parse().unwrap(){
                1 => 1.0,
                2 => 2.0,
                3 => 3.0,
                4 => 4.0,
                _ => {println!("Ingresa un numero dentro de los parámetros"); continue}
            };
            break; 
        };
        
    

        loop
        {
            let mut input = String::new(); 
            println!("¿Come algo durante sus comidas?\nOPCIONES:\n(0) Nunca\n(1) A veces\n(2) Frecuentemente\n(3) Siempre");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            caec = match input.trim().parse().unwrap(){
                0 => String::from("no"),
                1 => String::from("Sometimes"),
                2 => String::from("Frequently"),
                3 => String::from("Always"),
                _ => {println!("Ingresa un numero dentro de los parámetros"); continue}
            };
            break; 
        };
    
    

        loop
        {
            let mut input = String::new(); 
            println!("¿Usted fuma?\nOPCIONES:\n(0) SI\n(1) NO");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            smoke = match input.trim().parse().unwrap(){
                0 => String::from("yes"),
                1 => String::from("no"),
                _ => {println!("Ingresa un numero dentro de los parámetros"); continue}
            };
            break; 
        };
    

        loop
        {
            let mut input = String::new(); 
            println!("¿Cuanta agua bebe al día?\nOPCIONES:\n(0) Menos de 1 litro\n(1) Entre 1 y 2 litros\n(2) Más de 2 litros");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            ch20 = match input.trim().parse().unwrap(){
                0 => 1.0,
                1 => 2.0,
                2 => 3.0,
                _ => {println!("Ingresa un numero dentro de los parámetros"); continue}
            };
            break; 
        };
    
    

        loop
        {
            let mut input = String::new(); 
            println!("¿Monitorea sus calorías diariamente?\nOPCIONES:\n(0) SI\n(1) NO");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            scc = match input.trim().parse().unwrap(){
                0 => String::from("yes"),
                1 => String::from("no"),
                _ => {println!("Ingresa un numero dentro de los parámetros"); continue}
            };
            break; 
        };
        
    

        loop
        {
            let mut input = String::new(); 
            println!("¿Cuantas veces a la semana hace actividad física?\nOPCIONES:\n(0) Nunca\n(1) 1 o 2 dias\n(2) 2 o 4 dias\n(3) 4 o 5 dias");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            faf = match input.trim().parse().unwrap(){
                0 => 0.0,
                1 => 1.0,
                2 => 2.0,
                3 => 3.0,
                _ => {println!("Ingresa un numero dentro de los parámetros"); continue}
            };
            break; 
        };

        loop
        {
            let mut input = String::new(); 
            println!("¿Cuanto tiempo usa aparatos tecnológicos?\nOPCIONES:\n(0) 0 a 2 horas\n(1) 3 a 5 horas\n(2) Más de 5 horas");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            tue = match input.trim().parse().unwrap(){
                0 => 0.0,
                1 => 1.0,
                2 => 2.0,
                _ => {println!("Ingresa un numero dentro de los parámetros"); continue}
            };
            break; 
        };
        
    

        loop
        {
            let mut input = String::new(); 
            println!("¿Que tan amenudo ingiere alcohol?\nOPCIONES:\n(0) Nunca\n(1) A veces\n(2) Frecuentemente\n(3) Siempre");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            calc = match input.trim().parse().unwrap(){
                0 => String::from("no"),
                1 => String::from("Sometimes"),
                2 => String::from("Frequently"),
                3 => String::from("Always"),
                _ => {println!("Ingresa un numero dentro de los parámetros"); continue}
            };
            break; 
        };

        loop
        {
            let mut input = String::new(); 
            println!("¿Que medio de transporte normalmente usa?\nOPCIONES:\n(0) Automovil\n(1) Motocileta\n(2) Bicicleta\n(3) Transporte público\n(4) Caminando");
            io::stdin().read_line(&mut input).expect("Solo números dentro del rango establecido");
            mtran = match input.trim().parse().unwrap(){
                0 => String::from("Automobile"),
                1 => String::from("Motorbike"),
                2 => String::from("Bike"),
                3 => String::from("Public_Transportation"),
                4 => String::from("Walking"),
                _ => {println!("Ingresa un numero dentro de los parámetros"); continue}
            };
            break; 
        };
    
        Persona{
            gender ,
            age,
            altura,
            peso, 
            family_overweight,
            favc,
            fcvc, 
            ncp,
            caec,
            smoke,
            ch20,
            scc,
            faf,
            tue,
            calc,
            mtran,
            nobey: "".to_string()
        }
    }
    

}
