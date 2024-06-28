## Aesthetics
- [X] Pensar un diseño, para cuadrar márgenes y demás
- [X] Empty descuadra todo en función del ancho de la terminal
- [X] Diseño adaptable al tamaño de la terminal
    - [X] Diseño adaptable al iniciar el programa
- [X] Crear un thread que revise el tamaño de la terminal para redibujar en caso necesario
- [ ] Añadir colores personalizables a los bordes
- [ ] Permitir imprimir según qué bordes

## Dictionary
- [X] La manera de gestionar la memoria de los Strings es poco eficiente aunque funcione.
        Tiene mucha redundancia

## Cosas que hacer
- [X] Al borrar, marcar en gris si es correcto
- [X] Al escribir una palabra incorrecta, y escribir un caracter correcto, se marca como seleccionado y no como rojo
- [X] Utilizar líneas
- [X] Mostrar wpm al acabar líneas
- [X] Tests para ver que state no crashea en ningún momento
- [X] Tests para ver como realiza una prueba que sea siempre igual.
- [X] Para poder actualizar la pantalla, es necesario una nueva estructura, para cambiar el estado
      desde diferentes hilos sin afectar demasiado la implementación del main.
- [X] Mejorar representación interna del tiempo usando tipos propios de Rust
- [X] Agrupar toda la configuración en un struct dentro de State
- [X] Mejor interfaz para imprimir caracteres tanto de control como normales
- [X] Cambiar a crossterm
- [X] Refactor para modularizar State
    - [X] Separar control de estado de dibujado con el trait Display en State?
        - [X] Crear struct para la línea en sí del texto
        - [X] Abstraer en un struct diferente una línea, mostrandolas con Display
    - [X] Hacer un draw_box() con argumentos genéricos (posicion y tamaño)
    - [X] Unir funcion de highlight y color -> En realidad las he eliminado xd
    - [-] Sacar a un archivo diferente BoxInfo, Screen y WordState
        - [X] WordState sacado
        - [ ] Screen sacado
        - [X] BoxInfo sacado
- [X] Añadir modo File
- [ ] Añadir modo wikipedia, este modo coge un artículo de wikipedia que debe ser completado
- [X] Gestionar de manera correcta los diferentes modos
- [X] Tiempo de duración personalizable
    - [X] Tiempo añadido
- [-] Configuración
    - [ ] Crear archivo de configuración
    - [ ] Personalizar colores
    - [X] Cambiar número de lineas
    - [X] Cambiar ancho que se muestra
    - [ ] Cambiar modo
- [ ] Logs para debug
- [ ] Hacer mecano a prueba de errores
    - [ ] Archivos inexistentes o corruptos
    - [ ] Argumentos incorrectos
    - [ ] Escribir mientras aparece el mensaje TOO NARROW
    - [ ] Argumentos para crear State o Mecano incorrectos

## Hoja de ruta

- State necesita poder ubicarse para redimensionar
- State necesita dos líneas
- Como creo state para que cuadre con el modo
- Mecano para manejar el redimensionar
- Configuración extensible

## Restante para publicación

- [X] Modo archivo
- [-] CLI adecuado
    - [ ] Autocomplete
- [X] Configuración de los elementos ya previstos
    - [X] Leer config a partir de un archivo
    - [X] Añadir modo por defecto a config
    - [X] Toda la información para construir mecano debe tomarse de config
    - [X] Error para el usuario si no existe un archivo de configuración
- [ ] Empaquetado

## Explicación

- Pasos a seguir para completar el proyecto
    - Entender la terminal
    - Movimiento del cursor
    - Colores del cursor
    - Modo raw
    - Crear otro screen para mantener lo anterior (como nvim)
    - Tecleo en vivo
    - MecanoLine para abstraer
    - Base del state para una palabra
    - Modo debug para ver estado
    - Uso de los offsets
    - Diccionario de palabras
    - Ciclar líneas
    - Mejorar estética
        - Siempre centrado
    - Threads para ajustar tamaño y para controlar el tiempo
    - Sistema de puntuación
    - Histórico de puntuación
    - Modo texto
    - Menú para elegir modos o configuración
    - Configuración
    - Configuración de rutas para tomar configuraciones por defecto
    - Añadir licencia GNU
    - Empaquetar para subir a AUR
    - Subir a AUR
    - Subir a todos los repositorios que pueda
