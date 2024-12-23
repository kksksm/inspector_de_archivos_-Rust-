use std::fs;
    //detonador
    fn main(){
    while 1 == 1{
    println!("{:#?}", inicializacion());
    }}
    //arbol de desiciones
    fn inicializacion() -> Result<(),Box<dyn std::error::Error>>{
    let mut ingreso = String :: new();
    println!("minar/scrapt");
    let _ = std::io::stdin().read_line(&mut ingreso).unwrap();
    ingreso = ingreso.trim().to_string();
    if ingreso == "minar"{minar()}else{scrapt();
    Ok(())}
    }
    //funcion de minar archivos 
    fn minar() -> Result<(),Box<dyn std::error::Error>>{
    println!("ingrese la ruta que desea leer: ");
    let mut ruta = String :: new();
    std::io::stdin().read_line(&mut ruta).unwrap();
    let ruta = ruta.trim();
    let entries = fs::read_dir(ruta)?;
    for entry in entries{
    let entry = entry?;
    println!("({:?})", entry.path());
    }
    Ok(())}
    //funcion de scrapt
    fn scrapt(){
    let mut path = String :: new();
    println!("ingrese la ruta de acceso");
    let _ = std::io::stdin().read_line(&mut path);
    let path = path.trim();
    let archivo = fs::read_to_string(path).unwrap();
    println!("{}", archivo);
    }

