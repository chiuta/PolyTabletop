# Trei Căi (spune, mimează, desenează) (`treicai`)

Trei Căi: joc de explicat și ghicit în terminal: vorbit, mimă sau schiță, cu 218 acțiuni și meserii și aspecte predefinite.

## Scop

Adună mai multe puncte în 12 runde (rolurile alternează).

## Pregătire

Runda cere pe rând un mod diferit: vorbit (descrie cu cuvinte), mimă (alegi gesturi) sau schiță (alegi tușe). Pentru fiecare cuvânt-acțiune există aspecte predefinite; colegul-robot și adversarul ghicesc dintre 6 candidați. Motor comun `w4_ghici`.

## Desfășurarea unei ture

Descriptorul alege indicii (`arata N`). După fiecare indiciu adversarul poate ghici (`ghici N`) sau da `pas`; apoi colegul-robot (pas forțat `robot`) ghicește sau așteaptă. Ghicirea se verifică prin `verifica`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `arata 5` — arăți indiciul 5
* `ghici 3` — ca adversar, ghicești candidatul 3 (furi runda)
* `pas` — ca adversar, nu ghicești
* `robot` — pas forțat: colegul-robot ghicește
* `verifica` — pas forțat: verificare

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play treicai --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `arata 5` — arăți indiciul 5 („Se apasă un buton”).
2. `arata 2` — arăți indiciul 2 („Se face mult cu mâinile”).
3. `pas` — în runda următoare, ca adversar, dai `pas`.

## Reguli speciale și variante alese

Ghicirea greșită a adversarului costă −1 punct.

## Punctaj

Cu cât mai puține indicii, cu atât mai multe puncte (4…1); ghicire greșită a adversarului −1.

## Sfârșitul jocului

După 12 runde câștigă cine are mai multe puncte.

## Note despre AI și informația ascunsă

Colegul-robot ghicește după o euristică pe jetoanele indiciilor arătate, fără să vadă răspunsul.

## Diferențe față de regulile tradiționale

Variantă originală; cuvintele și aspectele sunt ale noastre.
