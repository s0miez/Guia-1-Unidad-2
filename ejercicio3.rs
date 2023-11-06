use std::default::Default;
mod biblioteca;
#[derive(Default)]
#[derive(Clone)]
struct Peliculas {
    titulo: String,
    director:String,
    anio_lanzamiento:i32
}

fn crear_arreglo() -> [Peliculas;4]{
    let arreglo:[Peliculas;4] = Default::default();
    return arreglo;

} 
fn leer_arreglo(cine: &[Peliculas;4]) -> (){
    for (num, obj) in cine.iter().enumerate() {
        print!("{}: Titulo de la pelicula: {} ", num+1, obj.titulo);
        print!("Director: {} \n", obj.director);
        print!("Año de lanzamiento: {:?}\n", obj.anio_lanzamiento);
    }
}

fn editar_arreglo(cine: [Peliculas;4]) -> [Peliculas;4]{
    let mut cine_edit :[Peliculas;4]= cine.clone(); 
    for i in 0..cine_edit.len() as usize{
        cine_edit[i].titulo = biblioteca::input_texto("Titulo de la pelicula".to_string());
        cine_edit[i].director = biblioteca::input_texto("Director".to_string());
        cine_edit[i].anio_lanzamiento = biblioteca::num("Año de lanzamiento".to_string());
    }
    return cine_edit;
}

fn buscador(cine: &[Peliculas;4]) ->() {
    loop{
        println!("¿Deseas saber qué peliculas se estrenaron en el año que digites?");
        if biblioteca::si_o_no() {
            let buscar = biblioteca::num("año de publicacion para buscar".to_string());
            for (i,x) in cine.iter().enumerate(){
                if cine[i].anio_lanzamiento == buscar {
                    print!(" Titulo de la pelicula: {} ", x.titulo);
                    print!("  Director: {}", x.director);
                    print!("    Año de lanzamiento: {:?}\n", x.anio_lanzamiento);
                }
            }
        } else {
            break
        }
}
}

fn main(){
    let arreglo = crear_arreglo();
    leer_arreglo(&arreglo);
    let arreglo = editar_arreglo(arreglo);
    buscador(&arreglo);
}
