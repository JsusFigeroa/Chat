Uso de docker.
Se construye la imagen desde la raíz del proyecto con 
    $ docker build -t nombre_de_img .
Para correr el programa hay que indicar cual es el ejecutable que se quiere ejecutar
ya que son dos programas Cliente y Servidor.
Para ejecutar el servidor:
    $ docker run -it --rm --name servidor -p 8080:8080 nombre_de_img cargo run --bin servidor
Nota: Al iniciar la ejecución pide un puerto, si se presiona enter se establece por defecto en 8080 con lo que solo sería
necesario establecer el puerto de la computadora para cambiar el puerto que usaran los cliente, es decir usar 1234:8080
para que los clientes accedan al servidor en el puerto 1234.
Para ejecutar el cliente:
    $ docker run -it --rm --name cliente nombre_de_img cargo run --bin cliente
Para detener cualquiera de los dos presionar Ctrl + C o en el caso especifico
del cliente existe el comando /disconnect .