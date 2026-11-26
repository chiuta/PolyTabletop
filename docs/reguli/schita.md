# Schița (desen simbolic) (`schita`)

Schița: joc de desenat și ghicit în terminal, cu 224 de cuvinte, fiecare cu 6 tușe textuale.

## Scop

Adună mai multe puncte în 10 runde.

## Pregătire

Desenatorul alege, una câte una, tușe dintr-un set de schițe textuale ale cuvântului secret (cerc, coadă, urechi ascuțite…). Colegul-robot și adversarul se uită la tușe și ghicesc dintre 6 candidați.

## Desfășurarea unei ture

`arata N` arată tușa N; adversarul poate fura ghicind (`ghici N`) sau `pas`; colegul-robot ghicește la pasul `robot`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `arata 2` — arăți tușa 2
* `ghici 3` — ca adversar ghicești candidatul 3
* `pas` — nu ghicești
* `robot` — pas forțat

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play schita --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `arata 2` — arăți tușa 2.
2. `arata 1` — arăți tușa 1.
3. `pas` — în runda următoare, ca adversar, dai `pas`.

## Reguli speciale și variante alese

Rolurile alternează.

## Punctaj

Cu cât mai puține tușe, cu atât mai multe puncte (5…1).

## Sfârșitul jocului

După 10 runde câștigă cine are mai multe puncte.

## Note despre AI și informația ascunsă

Colegul-robot ghicește după jetoanele tușelor arătate, fără să vadă răspunsul.

## Diferențe față de regulile tradiționale

Variantă originală; schițele sunt text, nu desen.
