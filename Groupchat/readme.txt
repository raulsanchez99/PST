Este proyecto implementa un servidor capaz de gestionar múltiples usuarios simultáneamente, permitir la creación y unión a grupos de chat, y el envío de mensajes en tiempo real entre los miembros.
Incluye control de usuarios mediante identificadores hash, gestión de grupos en memoria con estructuras sincronizadas (Arc<Mutex<...>>) y comunicación cliente-servidor basada en comandos.
