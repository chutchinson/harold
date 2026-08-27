mov rx, 0
mov ry, 1
mov rv, 10

loop:
    mov rz, rx       ;
    print            ;
    add              ; rz = rx + ry
    mov ry, rx       ; a = b
    mov ry, rz       ; b = next
    dec rz, rv       ;   rv -= 1, rz = rv
    jg 6             ;   loop while rz > 0
    hlt              ;

; document
; - label / comment / instruction