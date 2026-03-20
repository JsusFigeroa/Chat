Uso de docker.
Se construye la imagen desde la raíz del proyecto para cada uno de los ejecutables.
Para el servidor usamos:
    $ docker build -t img_servidor -f Dockerfile.servidor .
Para el cliente usamos: 
    $ docker build -t img_cliente -f Dockerfile.cliente .
Para ejecutar el servidor :
    $ docker run -it --rm --name servidor-chat -p 8080:8080 --init img_servidor
Para ejecutar el cliente:
    $ docker run -it --rm --init --name cliente-chat img_cliente