use core::str;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

use std::fs::File;
use std::io::Write;
//use colored::*;

//use std::str;
use std::thread;

use colored::Colorize;
//use std::time::Duration;

fn solicitud_respuesta(stream: &mut TcpStream, mensaje: &str)-> io::Result<()>{
    stream.write_all(mensaje.as_bytes())?;
    //println!("{}",mensaje);

    // Buffer para extraer el mensaje de respuesta, 
    let mut buffer = [0; 200];

    // Recibe la respuesta del servidor leyendo del stream
    let len = stream.read(&mut buffer)?;

    let respuesta = str::from_utf8(&buffer[..len])
        .expect("Respuesta no válida")
        .trim();

    // Parsear respuesta
    let partes: Vec<&str> = respuesta.splitn(2, " ").collect();

    match partes.as_slice() {
        [".info", texto] => println!("{}", texto.cyan()),
        _ => println!("Respuesta desconocida: {}", respuesta),   
    }

    Ok(()) 
}


fn main() -> std::io::Result<()> {
 
    // Crea un socket TCP para y abre la conexión con el servidor
    let mut stream = TcpStream::connect("127.0.0.1:8889")?;
    
    //
    let mut hash_cliente: Option<String> = None;

    println!(
        "Cliente se ha conectado desde {} al servidor: {}",
        stream.local_addr()?,
        stream.peer_addr()?
    );

    //NEWUSER
    loop {
        let mut comando = String::new();

        io::stdin().read_line(&mut comando)?;

        let comando = comando.trim();

        //Dividimos el comando introducido para poder extraer el nickname
        let msg: Vec<&str> = comando.split_whitespace().collect();

        match msg.as_slice() {
            [".newuser", user] => {
                //Obtenemos el nickname y lo enviamos al servidor
                let nickname = user.to_string();
                let mensaje = format!(".newuser {}\n", nickname);
                stream.write_all(mensaje.as_bytes())?;
                println!("{}", "Comprobando tu nickname con el servidor".yellow());
                
                //Esperamos la respuesta del servidor

                // Buffer para extraer el mensaje de respuesta,
                let mut buffer = [0; 100];

                // Recibe la respuesta del servidor leyendo del stream
                let len = stream.read(&mut buffer)?;

                //VALOR DE LA RESPUESTA
                let respuesta = str::from_utf8(&buffer[..len])
                    .expect("El mensaje no se puede convertir a string")
                    .trim();

                let partes: Vec<&str> = respuesta.split_whitespace().collect();

                match partes.as_slice() {
                    [".accept", nickname, hash] => {
                        println!("{}", format!("Tu nickname '{}' ha sido aceptado", nickname).green()); // verde
                        println!("{}", "Bienvenido a GroupChat".green());
                        
                        //Guardo el hash
                        hash_cliente = Some(hash.to_string());

                        //Nombre del fichero
                        let filename = format!("{}.txt", nickname);

                        //Creamos fichero para guardar la hash
                        let mut file = File::create(&filename)
                            .expect("Error al crear el archivo");

                        //Guardamos la hash en el    
                        file.write_all(hash.as_bytes())
                            .expect("Error al escribir en el archivo");

                    }
                    [".reject", nickname] => {
                        println!("{}", format!("El nickname '{}' ya está en uso. Saliendo ...", nickname).red());
                        return Ok(());
                    }
                    _ => {
                        println!("Respuesta del servidor no válida: {}", respuesta);
                        return Ok(());
                    }
                }

                break;
            }
            _ => {
                //Si no lo introduce correctamente, pedimos que introduzca correctamente
                println!("Debes registrarte: .newuser nickname");
            }
        } 
    }    
    
    
    let mut stream_respuesta = stream.try_clone()?;
    
    thread::spawn(move || {
        
        //Buffer para extraer el mensaje de respuesta
        let mut buffer = [0; 200];
        
        loop {
            match stream_respuesta.read(&mut buffer) {
                Ok(0) => break, // conexión cerrada
                
                Ok(len) => {
                    let mensaje = String::from_utf8_lossy(&buffer[..len]);
                    
                    if let Some(resto) = mensaje.strip_prefix(".info ") {
                        println!("{}", resto.cyan()); //mensaje informativo
                    } else {
                        println!("{}", mensaje); //mensaje de grupo
                    }
                }
                Err(_) => break,
            }
        }
    });


    loop{
        let mut comando = String::new();

        //Comando introducido por el usuario
        io::stdin().read_line(&mut comando)?;

        //Control de la entrada
        let comando = comando.trim();

        //Obtenemos mi hash
        let hash = hash_cliente.as_ref().unwrap();

        //Dividimos el comando introducido
        let msg: Vec<&str> = comando.split_whitespace().collect();

        match msg.as_slice() {
            [".list"] => {
                //Formo y envio el mensaje
                let mensaje = format!("{} .list", hash);
                solicitud_respuesta(&mut stream, &mensaje)?;
            }

            [".create", grupo] => {
                //Formo y envio el mensaje
                let mensaje = format!("{} .create {}", hash, grupo);
                solicitud_respuesta(&mut stream, &mensaje)?;
            }

            [".join", grupo] => {
                //Formo y envio el mensaje
                let mensaje = format!("{} .join {}", hash, grupo);
                solicitud_respuesta(&mut stream, &mensaje)?;
            }

            [".leave"] => {
                let mensaje = format!("{} .leave", hash);
                stream.write_all(mensaje.as_bytes())?;
                //solicitud_respuesta(&mut stream, &mensaje)?;
            }
            
            [".quit"] => {
                let mensaje = format!("{} .quit", hash);
                stream.write_all(mensaje.as_bytes())?;
                //println!("{}",mensaje);
                break;
            }

            [_, ..]=> {
                let texto= msg.join(" ");
                
                let mensaje = format!("{} {}",hash ,texto);
                stream.write_all(mensaje.as_bytes())?;
                //println!("{}",mensaje);


            }
            _ => {
                //-------TEXTO--------
                print!("Comando no conocido")
                
            }
        }


    }
    
    Ok(())
 
}
