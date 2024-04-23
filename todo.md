## Cosas que hacer
- [X] Al borrar, marcar en gris si es correcto
- [X] Al escribir una palabra incorrecta, y escribir un caracter correcto, se marca como seleccionado y no como rojo
- [ ] Utilizar líneas
- [ ] Mostrar wpm al acabar líneas
- [ ] Tests para ver que state no crashea en ningún momento
- [ ] Tests para ver como realiza una prueba que sea siempre igual

## Aesthetics
- [ ] Pensar un diseño, para cuadrar márgenes y demás
- [ ] Empty descuadra todo en función del ancho de la terminal
- [ ] Cuando la líneas es más larga que el ancho se descuadra todo
- [ ] Diseño adaptable al tamaño de la terminal
- [ ] Crear un thread que revise el tamaño de la terminal para redibujar en caso necesario


## Bugs
- [X] `is_typed_corrected()`: No usar slices, ya que corta caracteres de >1 byte
- [ ] Los caracteres que modifican el siguiente caracter, pero que no imprimen nada, desajustan los offsets
        
