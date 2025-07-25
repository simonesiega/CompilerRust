set -e

# 1. Compila il progetto Rust (compilatore)
echo "Compilazione del compilatore Rust..."
cargo build --release

# 2. Esegui il compilatore Rust per generare output.asm
echo "Esecuzione del compilatore per generare assembly..."
./target/release/compiler_rust

# 3. Assembla e linka l'assembly con NASM e ld
echo "Compilazione assembly con NASM..."
./assemble.sh out/output.asm