mov rx, 0
mov ry, 1
mov rv, 10
loop:
    mov rz, rx
    print 
    add 
    mov ry, rx
    mov rz, ry
    dec 
    jg loop
    hlt
