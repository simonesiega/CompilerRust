# Operatore di assegnamento `=`
L'operatore `=` è utilizzato per assegnare un valore a una variabile già dichiarata. A differenza dei linguaggi più semplici, in questo linguaggio l'assegnamento è considerato un **operatore con semantica definita**, e in futuro sarà implementabile come **funzione di sistema sovrascrivibile**.

---

## Uso di base
L'uso più comune dell'operatore `=` è quello di associare un valore a una variabile:
```plaintext
let x: i32 = 10;
x = 15;
```

Nel primo caso, il valore `10` viene assegnato al momento della dichiarazione. Nel secondo, viene effettuata una **riassegnazione**, permessa solo se la variabile è stata dichiarata con `mut`.

## Semantica interna

L'operatore `=` non è un’operazione elementare, ma rappresenta internamente una **funzione speciale di sistema** con la seguente forma astratta:
- `target`: riferimento mutabile alla variabile da modificare.
- `value`: nuovo valore da assegnare.
- Il tipo `T` deve rispettare le regole di compatibilità semantica (assegnabilità diretta o convertibilità).
- Il tipo di ritorno è `void`, ma in future estensioni può essere ridefinito.

---

## Override dell'operatore

In una fase avanzata del linguaggio, l’operatore di assegnamento sarà definibile (overrideabile) per **tipi complessi o personalizzati**, attraverso metodi speciali nelle classi di sistema.

Esempio concettuale futuro: