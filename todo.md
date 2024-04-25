## Cosas que hacer
- [X] Al borrar, marcar en gris si es correcto
- [X] Al escribir una palabra incorrecta, y escribir un caracter correcto, se marca como seleccionado y no como rojo
- [X] Utilizar líneas
- [ ] Mostrar wpm al acabar líneas
- [ ] Tests para ver que state no crashea en ningún momento
- [ ] Tests para ver como realiza una prueba que sea siempre igual. Con todos los carácteres UTF-8

## Aesthetics
- [ ] Pensar un diseño, para cuadrar márgenes y demás
- [ ] Empty descuadra todo en función del ancho de la terminal
- [ ] Cuando la líneas es más larga que el ancho se descuadra todo
- [-] Diseño adaptable al tamaño de la terminal
    - [X] Diseño adaptable al iniciar el programa
- [ ] Crear un thread que revise el tamaño de la terminal para redibujar en caso necesario

## Dictionary
- [X] La manera de gestionar la memoria de los Strings es poco eficiente aunque funcione.
        Tiene mucha redundancia

## Otros
- [ ] Para poder actualizar la pantalla, es necesario una nueva estructura, para cambiar el estado
      desde diferentes hilos sin afectar demasiado la implementación del main.

      struct Game {
        Mutex<State>,
      }

      Además es más escalable
- [X] Agrupar toda la configuración en un struct dentro de State
- [ ] Mejor interfaz para imprimir caracteres tanto de control como normales

## Bugs
- [X] `is_typed_corrected()`: No usar slices, ya que corta caracteres de >1 byte
- [X] Los caracteres que modifican el siguiente caracter, pero que no imprimen nada, desajustan los offsets
- [-] Tratar desbordes!
    - [ ] Para la información de debug cortar lo sobrante
    - [X] Para las líneas debo tratarlo de otra forma
- [ ] Al salir de la terminal, sigue en raw mode, por lo que queda inutilizable
- [ ] Cuando el ancho es inferior al width interno, panic
- [ ] Se pierden todos los highlight y colores al redibujar


## Hoja de ruta

- State necesita poder ubicarse para redimensionar
- State necesita dos líneas
- Como creo state para que cuadre con el modo
- Mecano para manejar el redimensionar
- Configuración extensible

## Explicación

- Pasos a seguir para completar el proyecto
    - Entender la terminal
    - Movimiento del cursor
    - Colores del cursor
    - Modo raw
    - Crear otro screen para mantener lo anterior (como nvim)
    - Tecleo en vivo
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
    - Empaquetar para subir a AUR
    - Subir a AUR
