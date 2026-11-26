# Scattergories (`scattergories`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play scattergories --seed 1`. Sursa implementării: `polytabletop-games/src/scattergories.rs`.

> Rezumat din `polytabletop list`: Joc de cuvinte pentru doi: 3 runde, fiecare cu o literă și 4 categorii (animal, mâncare, țară, obiect). Amândoi aleg în secret un cuvânt din listă; cuvintele identice dau 0, cele unice 1–3 (cu cât mai rar, cu atât mai mult). Mutări: caine, pas, arat.

## Scop

Aduni mai multe puncte după 12 categorii.

## Pregătire

3 litere trase; pentru fiecare, 4 categorii (animal, mâncare, țară, obiect); fiecare categorie are o listă de cuvinte de la comun la rar.

## Desfășurarea turei

Alegi în secret un cuvânt din listă (`caine`) sau `pas`, apoi J1 spune `arat`. Cuvânt unic = 1–3 puncte după raritate; identic = 0 pentru ambii.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `caine`
* `pas`
* `arat`

## Simplificări și note

Adaptare la alegere dintr-un dicționar (nu text liber). AI: speranța punctelor din popularitatea publică a cuvintelor; nu vede alegerea adversarului. Hot-seat: ascunde alegerea celuilalt.

## Informație ascunsă

Jocul are informație ascunsă: `render` arată doar ce are voie să vadă jucătorul care privește (în hot-seat, doar jucătorul la mutare), iar AI-ul nu citește informația ascunsă a adversarului.
