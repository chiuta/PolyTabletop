# Dame (8×8) (`draughts`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play draughts --seed 1`. Sursa implementării: `polytabletop-games/src/draughts.rs`.

> Rezumat din `polytabletop list`: Dame engleze 8×8: piesele simple mută și capturează doar înainte, regele (piesa care ajunge pe ultimul rând; mutarea se oprește acolo) un pas în toate cele patru diagonale. Captura e obligatorie și salturile continuă cât se poate. Mutări: c3-d4, c3xe5xg7.

## Scop

Capturezi toate piesele adversarului sau îl lași fără nicio mutare legală.

## Pregătire

Tablă 8×8, dame engleze: fiecare jucător are 12 piese pe pătratele închise din primele trei rânduri.

## Desfășurarea turei

Muți o piesă pe tură. Dacă există o captură, ești obligat să capturezi (nu poți alege o mutare simplă).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Mutare simplă: `c3-d4` (de la–la).
* Captură: `c3xe5`; salturile multiple se înlănțuie în aceeași mutare: `c3xe5xg7`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (7): `a3-b4`, `c3-b4`, `c3-d4`, `e3-d4`, `e3-f4`, `g3-f4`, `g3-h4`. Introduci `c3-d4`; răspunsul calculatorului (în ordine): `f6-e5`.
2. Mutări legale (1): `d4xf6`. Introduci `d4xf6`; răspunsul calculatorului (în ordine): `e7xg5`.
3. Mutări legale (7): `b2-c3`, `d2-c3`, `a3-b4`, `e3-d4`, `e3-f4`, `g3-f4`, `g3-h4`. Introduci `g3-h4`; răspunsul calculatorului (în ordine): `f8-e7`.

Notă: AI-ul acestui joc folosește un buget de timp sau simulări; răspunsurile calculatorului pot diferi de la o rulare la alta, chiar cu același `--seed`. Mutările legale și efectul mutărilor tale sunt deterministe.

## Reguli speciale și variante alese

* Piesele simple mută și capturează doar înainte; regele (piesa care ajunge pe ultimul rând) mută un pas în toate cele patru diagonale.
* Captura este obligatorie și salturile continuă cât se poate.
* O piesă simplă care ajunge pe ultimul rând devine rege și mutarea se oprește acolo (chiar dacă ar mai putea sări).

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Câștigă cel care lasă adversarul fără piese sau fără mutări legale. Remiză după 80 de mutări consecutive fără captură sau avansare (regulă a implementării).

## Note despre AI

Informație completă. Alpha-beta cu buget de timp (500 ms, adâncime maximă 16).

Stil AI în cod: alpha-beta cu adâncire iterativă și buget de timp (500 ms, adâncime maximă 16).

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Variantă engleză (americană) 8×8 cu regele de un pas (nu „zburător”), nu varianta internațională 10×10.
