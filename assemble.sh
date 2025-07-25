set -e

ASM_FILE="$1"
OBJ_FILE="${ASM_FILE%.asm}.o"
EXE_FILE=out/"output_exe"

# Compila .asm in .o
nasm -f elf64 -o "$OBJ_FILE" "$ASM_FILE"

# Linka .o in eseguibile
ld -o "$EXE_FILE" "$OBJ_FILE"

echo "Eseguibile creato: $EXE_FILE"