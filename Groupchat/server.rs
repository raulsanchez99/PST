use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;

use colored::*;
//use std::time::Duration;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

type Conexiones = Arc<Mutex<HashMap<u64, TcpStream>>>;
type Grupos = Arc<Mutex<HashMap<String, Vec<u64>>>>;
type Usuarios = Arc<Mutex<HashMap<u64, String>>>;


fn hash_string(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

fn handle_connection(mut stream: TcpStream, usuarios: Usuarios, grupos: Grupos, conexiones: Conexiones) -> std::io::Result<()> {
    println!("{}", format!("Cliente: {} se ha conectado", stream.peer_addr()?).yellow());

    loop {
        //Buffer para extraer el mensaje
        let mut buffer = [0; 200];

        let len = stream.read(&mut buffer)?;
        
        //Evitar mensajes vacios
        if len == 0 {
            break;
        }

        let mensaje = str::from_utf8(&buffer[..len])
            .expect("El mensaje no se puede convertir a string");

        let comando: Vec<&str> = mensaje.split_whitespace().collect();

        match comando.as_slice() {
            [".newuser", nickname] => {
                //Comprobamos si el nickname esta repetido
                let mut usuarios_guard = usuarios.lock().unwrap();

                if usuarios_guard.values().any(|n| n == nickname){
                    //NICKNAME REPETIDO
                    let respuesta = format!(".reject {}", nickname);
                    stream.write_all(respuesta.as_bytes())?;
                    println!("{}", "Respuesta enviada al cliente.".cyan());

                }else{
                    //NICKNAME NUEVO
                    let hash = hash_string(nickname);

                    //Guardamos usuario
                    usuarios_guard.insert(hash,nickname.to_string());

                    //Guardamos conexion
                    conexiones.lock().unwrap().insert(hash, stream.try_clone()?);

                    let respuesta = format!(".accept {} {}", nickname, hash);
                    stream.write_all(respuesta.as_bytes())?;
                    println!("{}", "Respuesta enviada al cliente.".cyan());
                }
            }

            [hash, ".list"] => {
                println!(".LIST del usuario con hash {}", hash);

                let grupos_guard = grupos.lock().unwrap();

                let texto = if grupos_guard.is_empty() {
                    "Lista de grupos: (vacia)".to_string()
                } else {
                    let mut nombres: Vec<String> = grupos_guard.keys().cloned().collect();
                    nombres.sort();
                    format!("Lista de grupos: {}", nombres.join(", "))
                };

                //Enviamos la respuesta al cliente
                let respuesta = format!(".info {}", texto);
                stream.write_all(respuesta.as_bytes())?;
                println!("{}", "[INFO] Respuesta enviada al cliente.".cyan());
            }

            [hash, ".create", grupo] => {
                println!(".CREATE {} del usuario con hash {}", grupo, hash);

                let mut grupos_guard = grupos.lock().unwrap();

                let texto = if grupos_guard.contains_key(*grupo) {
                    "Ese grupo ya esta creado".to_string()
                } else {
                    grupos_guard.insert(grupo.to_string(), Vec::new());
                    format!("Grupo {} creado", grupo)
                };

                let respuesta = format!(".info {}", texto);
                stream.write_all(respuesta.as_bytes())?;
                println!("{}", "[INFO] Respuesta enviada al cliente.".cyan());
            }

            [hash, ".join", grupo] =>{
                println!(".JOIN {} del usuario con hash {}", grupo, hash);

                let mut grupos_guard = grupos.lock().unwrap();

                //Comprobamos que el grupo exista
                let texto = if grupos_guard.contains_key(*grupo) {
                    //
                    let mut ocupado = false;
                    
                    //Convertimos el hash recibido a u64
                    let hash_u64: u64 = hash.parse().unwrap();
                    
                    //Comprobamos si el usuario esta en algun grupo
                    for (_, usuarios_grupo) in grupos_guard.iter() {
                        if usuarios_grupo.contains(&hash_u64){
                            ocupado = true;
                            break;
                        }
                    }

                    if ocupado{
                        "Tienes que salir primero de tu grupo actual".to_string()
                    }else{
                        //Añadimos el usuario al grupo
                        grupos_guard.get_mut(*grupo).unwrap().push(hash_u64);
                        format!("Te has unido al grupo {}", grupo)
                    } 
                    

                } else {
                    format!("El grupo {} no existe", grupo)
                };

                let respuesta = format!(".info {}", texto);
                stream.write_all(respuesta.as_bytes())?;
                println!("{}", "[INFO] Respuesta enviada al cliente.".cyan());
            }

            [hash, ".leave"] => {
                println!(".LEAVE del usuario con hash {}", hash);

                let mut grupos_guard = grupos.lock().unwrap();
                let usuarios_guard = usuarios.lock().unwrap();

                //Convertimos el hash recibido a u64
                let hash_u64: u64 = hash.parse().unwrap();

                //Obtenemos el nickanme del usuario
                let nickname = usuarios_guard.get(&hash_u64).unwrap().clone();

                //Condicion
                let mut grupo_ocupado: Option<String> = None;
                let mut participantes: Vec<u64> = Vec::new();
                

                //Comprobamos si el usuario esta en un grupo
                for (grupo, usuarios_grupo) in grupos_guard.iter_mut() {
                    if usuarios_grupo.contains(&hash_u64){
                        //Copiamos el nombre del grupo en el que estan
                        grupo_ocupado = Some(grupo.clone());
                        
                        //Eliminamos al usuario del grupo
                        usuarios_grupo.retain(|&u| u != hash_u64);

                        //Copiamos los usuarios del grupo
                        participantes = usuarios_grupo.clone();
                        break;
                    }
                }

                
                if let Some(_grupo) = grupo_ocupado{
                    // Si esta en un grupo
                    let mensaje_usuario = format!("{} ha salido del grupo", nickname);

                    let conexiones_guard = conexiones.lock().unwrap();

                    //NOtificamos al usuariode su salida
                    let respuesta = format!(".info Has salido del grupo");
                    let _ = stream.write_all(respuesta.as_bytes());

                    //Recorremos todos los usuarios del grupo
                    for hash_usuario in participantes{
                        // Notificamos a cada usuario del grupo
                        if let Some(stream) = conexiones_guard.get(&hash_usuario){
                            let mut stream = stream.try_clone().unwrap();
                            let _ = stream.write_all(mensaje_usuario.as_bytes());
                        }
                        
                    }
                }else{
                    let respuesta = format!(".info No estas en un grupo");
                    stream.write_all(respuesta.as_bytes())?;
                    println!("{}", "[INFO] Respuesta enviada al cliente.".cyan());
                }


                /*//Comprobamos si el usuario esta en un grupo
                for (grupo, usuarios_grupo) in grupos_guard.iter_mut() {
                    if usuarios_grupo.contains(&hash_u64){
                        grupo_ocupado = Some(grupo.clone());
                        //Eliminamos al usuario
                        usuarios_grupo.retain(|&u| u != hash_u64);
                        break;
                    }
                }
                
                let texto = if let Some(grupo) = grupo_ocupado {
                    format!("Te has salido del grupo {}", grupo)
                } else {
                    "No estas en ningun grupo".to_string()
                };

                let respuesta = format!(".info {}", texto);
                stream.write_all(respuesta.as_bytes())?;
                println!("{}", "[INFO] Respuesta enviada al cliente.".cyan());*/
            }

            [hash, ".quit"] => {
                println!(".QUIT del usuario con hash {}", hash);

                let mut grupos_guard = grupos.lock().unwrap();

                //Condicion
                let mut grupo_ocupado: Option<String> = None;
                let mut participantes: Vec<u64> = Vec::new();
                
                //Convertimos el hash recibido a u64
                let hash_u64: u64 = hash.parse().unwrap();

                //Comprobamos si el usuario esta en un grupo
                for (grupo, usuarios_grupo) in grupos_guard.iter_mut() {
                    if usuarios_grupo.contains(&hash_u64){
    
                        //Se elimina al usuario del grupo
                        usuarios_grupo.retain(|&u| u != hash_u64);

                        //Copiamos el nombre del grupo
                        grupo_ocupado = Some(grupo.clone());

                        //Copiamos el nombre de los otros usuarios del grupo
                        participantes = usuarios_grupo.clone();

                        break;
                    }
                }

                //Logica del caso .quit
                if let Some(grupo) = grupo_ocupado {
                    //Si habia usuarios en el grupo
                    if !participantes.is_empty() {


                        let usuarios_guard= usuarios.lock().unwrap();

                        //Guardamos el nickname del usuario que salio
                        let nickname = usuarios_guard.get(&hash_u64).unwrap().clone();

                        //Creamos el mensaje a enviar
                        let mensaje = format!("{} se ha ido del grupo {}", nickname, grupo);

                        let conexiones_guard = conexiones.lock().unwrap();

                        //Enviamos el mensaje
                        for usuario in &participantes{
                            if let Some(mut stream) = conexiones_guard.get(usuario){
                                let _ = stream.write_all(mensaje.as_bytes());
                            }
                        }
                        

                    }

                }else{
                    break;
                }

                
            }

            [hash, ..] =>{
                //Separamos el texto completo del hash
                let mensaje = &mensaje[hash.len()+1..];

                let mut grupos_guard = grupos.lock().unwrap();
                let usuarios_guard = usuarios.lock().unwrap();

                //Convertimos el hash recibido a u64
                let hash_u64: u64 = hash.parse().unwrap();

                //Obtenemos el nickanme del usuario
                let nickname = usuarios_guard.get(&hash_u64).unwrap().clone();
                
                //Condicion
                let mut grupo_ocupado: Option<String> = None;
                let mut participantes: Vec<u64> = Vec::new();

                println!("Mensaje de {}: {}", nickname, mensaje);

                //Comprobamos si el usuario esta en un grupo
                for (grupo, usuarios_grupo) in grupos_guard.iter_mut() {
                    if usuarios_grupo.contains(&hash_u64){
                        //COpiamos el nombre del grupo en el que estan
                        grupo_ocupado = Some(grupo.clone());
                        
                        //Copiamos el nombre de los otros usuarios del grupo
                        participantes = usuarios_grupo.clone();
                        break;
                    }
                }
                
                if let Some(_grupo) = grupo_ocupado{
                    let mensaje_usuario = format!("{}: {}", nickname, mensaje);

                    let conexiones_guard = conexiones.lock().unwrap();

                    for hash_usuario in participantes{
                        if hash_usuario != hash_u64{
                            if let Some(stream) = conexiones_guard.get(&hash_usuario){
                                let mut stream = stream.try_clone().unwrap();
                                let _ = stream.write_all(mensaje_usuario.as_bytes());
                            }
                        }
                    }
                }else{
                    let respuesta = format!(".info Para mandar un mensaje debes estar en un grupo");
                    stream.write_all(respuesta.as_bytes())?;
                    println!("{}", "[INFO] Respuesta enviada al cliente.".cyan());
                }
                

            }

            _ => {
                println!("Comando no reconocido");
                break;
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let usuarios: Usuarios = Arc::new(Mutex::new(HashMap::new()));
    let grupos: Grupos = Arc::new(Mutex::new(HashMap::new()));
    let conexiones: Conexiones = Arc::new(Mutex::new(HashMap::new()));

    // Crea un socket TCP para escuchar conexiones entrantes en el puerto 8888
    let listener = TcpListener::bind("0.0.0.0:8889")?;
    println!("{}", "Esperando conexiones...".green());

    // Para cada conexión entrante, lee el mensaje y responde
    for result_stream in listener.incoming() {
        let stream = result_stream?;

        // clonamos el almacén lo que simplemente aumenta su contador de referencias
        let usuarios = Arc::clone(&usuarios);
        let grupos = Arc::clone(&grupos);
        let conexiones = Arc::clone(&conexiones);

        thread::spawn(move || -> std::io::Result<()> {
            handle_connection(stream, usuarios, grupos, conexiones)?;
            Ok(())
        });
    }

    Ok(())
}
