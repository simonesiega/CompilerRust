# La keyword `let`
La keyword `let` è utilizzata per dichiarare una nuova variabile nello scope corrente, associando un valore iniziale a un identificatore con un tipo esplicitamente definito. È il costrutto fondamentale per introdurre dati e mantenerli coerenti all'interno di un blocco di codice.

---

## Sintassi
```
let [mut] <nome_variabile>: <tipo> = <valore_iniziale>;
```

### Componenti della sintassi
- `let`: keyword obbligatoria per la dichiarazione.
- `mut` (opzionale): rende la variabile mutabile.
- `<nome_variabile>`: identificatore univoco, conforme alle regole di naming.
- `<tipo>`: tipo esplicito della variabile (es. `i32`, `f64`, `bool`, ecc.).
- `<valore_iniziale>`: valore iniziale che deve essere specificato obbligatoriamente.

## Uso di base
Il pattern più comune è una singola variabile, dove l'espressione fornita viene associata alla variabile specificata:
```plaintext
let numero: int = 100;
let somma: int = 200 + numero;

let mut variabile_modificabile: bool = true;
variabile_modificabile = false;
```

## Tipizzazione obbligatoria
Ogni variabile deve essere dichiarata con un tipo esplicito. Non è prevista inferenza automatica:
```plaintext
// Corretto - tipo specificato
let valore: int = 42;

// Errore - tipo mancante
// let valore = 42;
```

## Inizializzazione obbligatoria
Tutte le variabili devono essere inizializzate al momento della dichiarazione per garantire:
- **Prevenzione di errori** legati all'uso di variabili non inizializzate
- **Ottimizzazione delle risorse** evitando allocazioni inutili
- **Maggiore sicurezza** del codice eliminando stati indefiniti
```plaintext
// Dichiarazioni valide
let eta: int = 25;
let nome: string = "Mario";
let attivo: bool = true;

// Dichiarazioni non valide - mancata inizializzazione
let eta: int;
let nome: string;
```

## Shadowing
È consentito dichiarare una nuova variabile con lo stesso nome di una esistente all’interno dello stesso scope o in uno scope annidato. Questo comportamento è detto *shadowing* e crea una nuova istanza del nome, non una modifica della variabile originale.
```plaintext
let valore: i32 = 10;
{
    let valore: string = "test";
    // In questo blocco, valore è di tipo string
}

// Fuori dal blocco, valore è di nuovo i32
```

## Scope e visibilità
Le variabili seguono regole di visibilità basate sullo scope:

- Le variabili dichiarate all'interno di un blocco `{}` sono accessibili solo in quel blocco.
- Le variabili esterne possono essere shadowate da nuove dichiarazioni interne con lo stesso nome.
- Al termine del blocco, le variabili dichiarate al suo interno vengono deallocate.
```plaintext
let variabile_esterna: int = 10;
{
    let variabile_locale: int = 20;
    let variabile_esterna: string = "locale"; // Shadowing
    // Qui variabile_esterna è di tipo string
} 
// variabile_locale esce dallo scope
// Qui variabile_esterna torna ad essere di tipo int con valore 10
```