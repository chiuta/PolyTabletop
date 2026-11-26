# Dicționar Viu (explică în rafală) (`dictionar`)

Dicționar Viu: joc de explicat cuvinte în rafală, cu 215 cuvinte cu trei explicații predefinite.

## Scop

Fă cât mai multe puncte în 6 ture (rolurile alternează).

## Pregătire

Fiecare tură are un buget de 9 acțiuni (un indiciu arătat sau `treci` costă câte o acțiune). Cuvintele curg unul după altul.

## Desfășurarea unei ture

Descriptorul arată indicii (`arata N`) sau `treci`; colegul-robot ghicește (`robot`) sau adversarul fură (`ghici N`).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `arata 1` — arăți explicația 1
* `treci` — treci la cuvântul următor (costă o acțiune)
* `ghici 2` — ca adversar ghicești
* `pas` — nu ghicești
* `robot` — pas forțat

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play dictionar --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `arata 1` — arăți explicația 1.
2. `arata 1` — explicația 1 a cuvântului următor.
3. `arata 2` — explicația 2.

## Reguli speciale și variante alese

Nu ai voie să folosești rădăcina cuvântului (faul −1).

## Punctaj

Fiecare cuvânt ghicit de colegul-robot sau furat de adversar valorează 1 punct.

## Sfârșitul jocului

După 6 ture câștigă cine are mai multe puncte.

## Note despre AI și informația ascunsă

Colegul-robot ghicește după jetoane, fără să vadă răspunsul.

## Diferențe față de regulile tradiționale

Variantă originală; „în rafală” = mod fără rundă pe cuvânt.
