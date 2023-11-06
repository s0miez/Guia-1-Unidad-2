use std::default::Default;
mod biblioteca;
#[derive(Default)]
#[derive(Clone)]
struct Libro {
    titulo: String,
    autor: String,
    anio: i32,
}

fn crea_arreglo_libro() -> [Libro;10]{
    let arreglo: [Libro; 10] = Default::default();
    return arreglo;
}

fn leer_arreglo_libro(libros: &[Libro; 10]) -> () {
    for (num, item) in libros.iter().enumerate() {
        print!("{}: Título: {} ", num+1, item.titulo);
        print!("Autor: {} ", item.autor);
        print!("Anio de publicación: {}\n", item.anio);
    }
}

fn editar_arreglo_libro(libros: [Libro; 10]) -> [Libro; 10]{
    let mut libro_editado: [Libro; 10] = libros.clone();
    for i in 0..libro_editado.len(){
        libro_editado[i].titulo = biblioteca::input_texto("Título".to_string());
        libro_editado[i].autor = biblioteca::input_texto("Autor".to_string());
        print!("Ingrese el anio de publicación: ");
        libro_editado[i].anio = biblioteca::num("año de publicacion".to_string());
}
    return libro_editado
}

fn main() {
    let arreglo = crea_arreglo_libro();
    leer_arreglo_libro(&arreglo);
    println!(":");
    let arreglo = editar_arreglo_libro(arreglo);
    leer_arreglo_libro(&arreglo);
}
