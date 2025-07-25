use std::fs;
use std::io::Write;

fn main() {
    // Crea la cartella out se non esiste
    fs::create_dir_all("out").unwrap();

    // Crea il file out/output.asm
    let mut file = fs::File::create("out/output.asm").unwrap();

    // Scrive un semplice programma assembly NASM per stampare Hello, world!
    let asm_code = r#"
    section .data
        message db 'Hello, world!', 0xA
        len equ $ - message
    
    section .text
        global _start
    
    _start:
        mov rax, 1          ; syscall: sys_write
        mov rdi, 1          ; file descriptor: stdout
        mov rsi, message    ; messaggio da scrivere
        mov rdx, len        ; lunghezza del messaggio
        syscall             ; chiamata di sistema
    
        mov rax, 60         ; syscall: sys_exit
        xor rdi, rdi        ; exit code 0
        syscall             ; chiamata di sistema
    "#;

    file.write_all(asm_code.as_bytes()).unwrap();

    println!("File out/output.asm creato con successo!");
}
