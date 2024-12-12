fn main() {
    println!("Digite su nombre porfavor:");

    let mut nombre : String = String::new();

    std::io::stdin().read_line(&mut nombre).unwrap();

    println!("Hola {}",nombre);
}
