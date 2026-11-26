# Cuvinte Oprite (descrie fără cuvinte interzise) (`oprite`)

Cuvinte Oprite: descrie un cuvânt fără cuvintele interzise; 233 de cărți cu cuvânt-țintă, 4 interdicții și indicii-propoziție predefinite.

## Scop

Adună mai multe puncte în 10 runde.

## Pregătire

Descriptorul vede cuvântul și interdicțiile; indiciile care calcă o interdicție (sau cuvântul-țintă) sunt fauluri (−1, detectate automat). Colegul-robot și adversarul ghicesc dintre 6 candidați.

## Desfășurarea unei ture

Descriptorul alege indicii (`arata N`); adversarul poate fura ghicind (`ghici N`) sau `pas`; colegul-robot ghicește la pasul forțat `robot`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `arata 1` — arăți indiciul 1
* `ghici 4` — ca adversar ghicești candidatul 4
* `pas` — nu ghicești
* `robot` — pas forțat
* `verifica` — pas forțat

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play oprite --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `arata 1` — arăți indiciul 1 („Se rotește în jurul unei stele”).
2. `pas` — dai `pas` (nu ghicești) ca adversar.
3. `pas` — `pas` din nou.

## Reguli speciale și variante alese

Faulurile costă 1 punct.

## Punctaj

Cu cât mai puține indicii, cu atât mai multe puncte (`max_hints + 1 − indicii`); faul −1.

## Sfârșitul jocului

După 10 runde câștigă cine are mai multe puncte.

## Note despre AI și informația ascunsă

Colegul-robot ghicește după jetoanele indiciilor, fără să vadă răspunsul.

## Diferențe față de regulile tradiționale

Variantă originală; cuvintele, interdicțiile și indiciile sunt ale noastre.
