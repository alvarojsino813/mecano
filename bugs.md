- [X] `is_typed_corrected()`: No usar slices, ya que corta caracteres de >1 byte
- [X] Los caracteres que modifican el siguiente caracter, pero que no imprimen nada, desajustan los offsets
- [X] Tratar desbordes!
    - [X] Para las líneas debo tratarlo de otra forma
- [X] Al salir de la terminal, sigue en raw mode, por lo que queda inutilizable
- [X] Cuando el ancho es inferior al width interno, panic
- [X] Se pierden todos los highlight y colores al redibujar
- [X] Cuando el width no cabe en la pantalla, panic
- [X] Al hacer mucho zoom, panic (relacionado con el anterior)
- [X] El tiempo sigue apareciendo en la pantalla final de WPM
- [X] No se imprime la puntuación al final y se queda pillado
- [X] Cuando la líneas es más larga que el ancho se descuadra todo
- [X] Arreglar Backspace
- [X] Mejorar expresión de `typed` en `line.rs`
- [X] Arreglar tiempo de `Mecano`, que se ejecuta muchas veces por segundo en vez de una
- [X] No funciona la muestra de un número de líneas diferente a 2
- [X] No funcionan las tildes cuando son incorrectas
- [X] No se borra correctamente la línea anterior
- [X] `draw_box` no dibuja correctamente
- [X] Al hacer zoom y paralizar el tiempo, se añade tiempo al actualizar los segs a 0 y no guardar las décimas
- [X] Panic para `delta_time` > `1 / fps`
- [X] Solo se actualizan los marcadores por cada línea
- [X] No hay límite a la longitud del input, provocando varios bugs
- [X] Al hacer zoom se borra la palabra que estaba escrita
- [X] Al iniciar con tamaño menor al posible -> panic
- [X] Panic al ser muy aplanado
- [X] Cuando se muestra TOO NARROW se puede seguir escribiendo y no cuenta el tiempo
- [X] El primer y el último segundo se cuentan como segundos enteros, debería contar solo 1
- [X] Panic cuando el width es muy bajo
- [X] Debe haber un diccionario por defecto para funcionar
- [X] `lines_to_show` a 0. No se permite
- [X] width muy bajo -> panic
- [X] Líneas sin palabras -> panic
- [X] `lines_to_show` muy alto -> panic
- [X] `print_lines` debe adaptarse al `lines_to_show`
- [X] Líneas de una palabra no cuentan en la puntuación
- [X] Mostrar campos inválidos al introducir configuración inválida
    - [X] Deletrear mal el modo en `mecano.toml`
    - [X] Archivo no válido en `mecano.toml`
- [X] -l no imprime de la carpeta general si .config falla
- [X] si .config/mecano no está creado falla