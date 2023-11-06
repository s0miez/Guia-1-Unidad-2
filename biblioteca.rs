use std::io;

pub fn numero(variable:String) -> i32 {
    loop {
        println!("Digite un número para su {}: ", variable);
        let mut num = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut num).unwrap();
        let num: i32 = match num.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Digite un número válido.");
                continue;
            },
        };
        return num;
    }
}

pub fn num_float() -> f32 {
    loop {
        println!("Digite un número: ");
        let mut num = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut num).unwrap();
        let num: f32 = match num.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Digite un número válido.");
                continue;
            },
        };
        if num <= 7.0{
            println!("");
            return num;
        } else {
            println!("Digite un numero entre 1 - 7");
        }
    }
}



pub fn input_texto(variable: String) -> String {

    println!("Digite su {}", variable);
    let mut texto = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut texto).unwrap();
    return texto

}


pub fn si_o_no() -> bool {

    loop{
        println!("Digite 1 = SI, 0 = NO\n");
        let decision = num("decisión".to_string());
        if decision == 1 {
            return true;
        }
        if decision == 0{
            return false;
        } 
    }
}
