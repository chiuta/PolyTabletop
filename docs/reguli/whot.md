# Whot (cărți nigeriene) (`whot`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play whot --seed 1`. Sursa implementării: `polytabletop-games/src/whot.rs`.

> Rezumat din `polytabletop list`: Pachet de 54 de cărți (forme o t x p s, plus 5 Whot), 6 în mână. Joci aceeași formă sau același număr; Whot cere o formă. 1 și 8: mai joci o dată; 14: adversarul trage o carte; 2: trage 2, 5: trage 3 (se pot înlănțui). Câștigi golindu-ți mâna. Informație ascunsă: vezi mâna Jucătorului 1. Mutări: o5, t14, w:s, trage, pas.

## Scop

Rămâi primul fără cărți în mână.

## Pregătire

Pachet de 54 de cărți: cercuri `o` și triunghiuri `t` (1–5, 7, 8, 10–14), cruci `x` și pătrate `p` (1–3, 5, 7, 10, 11, 13, 14), stele `s` (1–5, 7, 8) și 5 cărți Whot (20). Fiecare primește 6 cărți; se întoarce o carte de start obișnuită.

## Desfășurarea turei

Joci o carte de aceeași formă sau același număr cu cea de deasupra; Whot se joacă oricând și cere o formă. Dacă nu poți joci, tragi o carte (`trage`) și tura trece.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Carte: `o5`, `t14` (formă + număr).
* Whot cu forma cerută: `w:s`.
* `trage` (piața), `pas` (doar dacă nu mai e nimic de tras).

## Reguli speciale și variante alese

* 1 (hold on) și 8 (suspendare): mai joci o dată.
* 14 (piață generală): adversarul trage o carte și mai joci o dată.
* 2: adversarul trage 2; 5: trage 3. Penalizările se pot înlănțui cu aceeași carte (2 pe 2, 5 pe 5); cine nu răspunde trage tot.

## Punctaj

Nu există punctaj pe rânduri (o singură mână).

## Sfârșitul jocului

Câștigă primul care golește mâna (și cu o carte specială). Dacă piața se golește, se amestecă teancul de jos.

## Note despre AI

Informație ascunsă: AI-ul evaluează doar mâna lui și numărul cărților adversarului; cartea abia trasă nu e folosită la alegere. Alpha-beta cu adâncime 1.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; mâna afișată este a jucătorului la mutare, cu un banner de predare a ecranului.

## Diferențe față de regulile tradiționale

* Fără „last card / check up” anunțat și fără punctaj; variantele regionale (număr de cărți împărțite, apărarea cu 2/5) diferă, s-a ales cea de mai sus.
