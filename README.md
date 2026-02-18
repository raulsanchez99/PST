# GroupChat en Rust

Aplicación de chat multicliente desarrollada en **Rust** que utiliza **sockets TCP** y **programación concurrente**.
El sistema permite a varios usuarios conectarse a un servidor, crear o unirse a grupos y enviar mensajes en tiempo real.

Este proyecto está orientado a practicar conceptos fundamentales de redes, concurrencia y gestión de memoria segura en Rust.

---

## Características

* Registro de usuarios mediante nickname.
* Identificación de clientes mediante hash.
* Creación y listado de grupos de chat.
* Unión y salida de grupos.
* Envío de mensajes en tiempo real entre usuarios del mismo grupo.
* Manejo de múltiples clientes simultáneamente mediante hilos.
* Sincronización de datos compartidos usando `Arc<Mutex<...>>`.

---

## Tecnologías utilizadas

* Lenguaje **Rust**
* Sockets TCP (`std::net`)
* Multithreading (`std::thread`)
* Sincronización con `Arc` y `Mutex`
* Manejo de colecciones con `HashMap`

---

## Arquitectura

El sistema está compuesto por dos programas:

### Servidor

* Escucha conexiones TCP entrantes.
* Gestiona usuarios, grupos y conexiones activas.
* Reenvía mensajes entre usuarios del mismo grupo.

### Cliente

* Se conecta al servidor.
* Permite ejecutar comandos para interactuar con el sistema.
* Muestra mensajes del grupo en tiempo real.

---

## Comandos disponibles

| Comando               | Descripción                      |
| --------------------- | -------------------------------- |
| `.newuser <nickname>` | Registra un nuevo usuario        |
| `.list`               | Muestra los grupos disponibles   |
| `.create <grupo>`     | Crea un nuevo grupo              |
| `.join <grupo>`       | Se une a un grupo                |
| `.leave`              | Sale del grupo actual            |
| `.quit`               | Sale del chat                    |
| `<mensaje>`           | Envía un mensaje al grupo actual |

---

## Ejecución del proyecto

### 1. Compilar el servidor

```bash
cargo build --bin servidor
```

### 2. Ejecutar el servidor

```bash
cargo run --bin servidor
```

### 3. Compilar el cliente

```bash
cargo build --bin cliente
```

### 4. Ejecutar uno o varios clientes

```bash
cargo run --bin cliente
```

---

## Ejemplo de uso

1. Iniciar el servidor.
2. Abrir dos terminales con clientes.
3. Registrar usuarios:

   ```
   .newuser Alice
   .newuser Bob
   ```
4. Crear y unirse a un grupo:

   ```
   .create general
   .join general
   ```
5. Enviar mensajes:

   ```
   Hola a todos
   ```
6. Salir del grupo
  ```
  .leave
  ```

7. Cerrar sesion
  ```
  .quit
  ```
