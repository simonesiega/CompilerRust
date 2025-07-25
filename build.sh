set -e

# Crea la cartella out se non esiste
mkdir -p out

echo "[1] Compilazione compilatore..."
cargo build --release

echo "[2] Esecuzione compilatore..."
./target/release/compiler_rust

echo "[3] Compilazione NASM..."
nasm -f elf64 out/out.asm -o out/out.o

echo "[4] Linking..."
ld out/out.o -o out/output

echo "[5] Esecuzione:"
./out/output