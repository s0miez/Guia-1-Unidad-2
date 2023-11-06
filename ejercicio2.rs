use std::default::Default;
mod biblioteca;
#[derive(Default)]
#[derive(Clone)]
struct Estudiantes{
    nombre:String,
    matricula: i32,
    notas:[f32;5]
}

fn crear_arreglo() -> [Estudiantes;4]{
    let arreglo:[Estudiantes;4] = Default::default();
    return arreglo;
}

fn leer_arreglo(estudiante: &[Estudiantes;4]) -> (){
    for (number, item) in estudiante.iter().enumerate() {
        print!("{}: Nombre: {} ", number+1, item.nombre);
        print!("Matricula: {} \n", item.matricula);
        print!("Notas {:?}\n", item.notas);
    }
}

fn editar_arreglo(estudiante: [Estudiantes;4]) -> [Estudiantes;4]{
    let mut estudiante_nuevo :[Estudiantes;4]= estudiante.clone(); 
    let mut promedio:f32 = 0.0;
    let mut nuevo_numero:f32 = 0.0;
    for i in 0..estudiante_nuevo.len() as usize{
        promedio = 0.0;
        estudiante_nuevo[i].nombre = biblioteca::input_texto("Nombre".to_string());
        println!("Número de matricula\n");
        estudiante_nuevo[i].matricula = biblioteca::num("Matricula".to_string());
        println!("Notas\n");
    
        for x in 0..estudiante_nuevo[i].notas.len() as usize{
            nuevo_numero = biblioteca::num_float();
            estudiante_nuevo[i].notas[x] = nuevo_numero;
            promedio += nuevo_numero;
        }
        let promedio:f32 = promedio / 5.0;
        println!("El promedio del estudiante es: {}", promedio);
        if promedio >= 4.0 {
            println!("El estudiante aprobó");
        } else {
            println!("El estudiante reprobó");
        }
    }
    return estudiante_nuevo;
}

fn main(){
    let arreglo = crear_arreglo();
    let arreglo = editar_arreglo(arreglo);
    leer_arreglo(&arreglo);
}
