# Plăcuțe (seturi și serii) (`placute`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play placute --seed 1`. Sursa implementării: `polytabletop-games/src/placute.rs`.

> Rezumat din `polytabletop list`: Variantă originală: 106 plăci (4 culori ×1–13 ×2 + 2 jokeri), 14 în mână. Pune serii/grupuri (pune r3 r4 r5), adaugă la masă (adauga r6 la 2), schimbă un joker (schimb J din 2 cu r4); prima ieșire ≥ 30. Terminare: gata; altfel trag / pas; anuleaza. Fără reorganizarea liberă a mesei. Informație ascunsă: vezi doar mâna ta.

## Scop

Rămâi primul fără plăci în mână.

## Pregătire

106 plăci: 4 culori (r roșu, a albastru, n negru, g galben) × numerele 1–13 × 2 exemplare + 2 jokeri (`J`). Fiecare primește 14; restul formează pachetul (ascuns). Începe J1.

## Desfășurarea turei

În tura ta poți, în orice ordine: să pui o combinație nouă, să adaugi o placă la capătul unei serii sau la un grup, sau să schimbi un joker de pe masă cu placa reală pe care o înlocuiește (jokerul luat trebuie jucat în aceeași tură). Prima ieșire a fiecărui jucător cere combinații noi cu suma ≥ 30 în aceeași tură și nu permite adăugări/schimburi. Termini tura cu `gata` (după cel puțin o placă jucată) sau tragi o placă (`trag`) dacă n-ai jucat nimic.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Serie: `pune r3 r4 r5`; grup: `pune r7 a7 n7`.
* `adauga r6 la 2` (la combinația 2); la joker: `adauga J la 2 inceput|sfarsit`.
* `schimb J din 2 cu r4`.
* `gata`, `trag`, `pas` (cu pachetul gol), `anuleaza` (readuce masa și mâna la începutul turei, după care poți doar trage).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (2): `pune a4 n4 g4`, `trag`. Introduci `pune a4 n4 g4`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1): `anuleaza`. Introduci `anuleaza`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (1): `trag`. Introduci `trag`; răspunsul calculatorului (în ordine): `pune r10 n10 g10`; `gata`.

## Reguli speciale și variante alese

* Serii: cel puțin 3 plăci consecutive de aceeași culoare; grupuri: 3–4 plăci cu același număr și culori diferite. Un joker ține locul oricărei plăci (o combinație trebuie să aibă măcar 2 plăci reale).

## Punctaj

La blocaj, punctele din mână (joker = 30).

## Sfârșitul jocului

Câștigă primul care rămâne fără plăci; după două pasuri la rând, câștigă cine are mai puține puncte în mână.

## Note despre AI

Informație ascunsă: afișajul arată mâna ta și mărimea mâinii adversarului. AI-ul citește doar masa, mâna lui și numărul plăcilor adversarului (placa trasă în ultima mutare nu intră în evaluare).

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Fără reorganizarea liberă a mesei (despărțiri/recombinări); doar adăugări și schimbul de jokeri.
