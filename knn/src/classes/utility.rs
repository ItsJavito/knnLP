//Funciones implementadas para obtener maximo y minimo, en rust no hay funciones implementadas para f32
pub fn get_max(ar: &Vec<f32>) -> f32 {
    let mut max = f32::MIN;
    for &i in ar {
        if i > max {
            max = i;
        }
    }
    max
}

pub fn get_min(ar: &Vec<f32>) -> f32 {
    let mut min = f32::MAX;
    for &i in ar {
        if i < min {
            min = i;
        }
    }
    min
}

pub fn get_min_max(ar: &Vec<f32>) -> (f32, f32) {
    let min = get_min(ar);
    let max = get_max(ar);
    (min, max)
}

pub fn categoric_data(data: String) -> f32 {
    let res: f32 = match data.as_ref() {
        "no" => 0.0,
        "Sometimes" => 0.3333,
        "Frequently" => 0.6666,
        "Always" => 1.0,
        _ => {
            println!("diff");
            0.0
        }
    };
    res
}

pub fn trans_data(data: String) -> f32 {
    let res: f32 = match data.as_ref() {
        "Walking" => 0.0,
        "Bike" => 0.25,
        "Public_Transportation" => 0.5,
        "Motorbike" => 0.75,
        "Automobile" => 1.0,
        _ => {
            println!("diff");
            0.0
        }
    };
    res
}
