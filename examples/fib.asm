; fibonacci

set rx, 0
set ry, 1
set rv, 10

loop:
    mov rz, rx       ;
    print rz         ;
    add              ; rz = rx + ry
    mov ry, rx       ; a = b
    mov ry, rz       ; b = next
    dec rz, rv       ;   rv -= 1, rz = rv
    jg 6             ;   loop while rz > 0
    hlt              ;


; document
; - label / comment / instruction