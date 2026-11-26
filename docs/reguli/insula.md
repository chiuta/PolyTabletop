# Insula (colonizare cu resurse) (`insula`)

Categorie: Strategie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play insula --seed 1`. Sursa implementării: `polytabletop-games/src/insula.rs`.

> Rezumat din `polytabletop list`: Joc original pe 19 hexagoane (pădure l, argilă a, pășune p, lan g, munte m) cu numere 2–12: pui colonii și drumuri (colonie v12, drum e7), arunci zarurile (arunc) ca să primești resurse, la 7 mută tâlharul (bandit h5), construiești colonii/orașe/drumuri, schimbi 4:1 cu banca (schimb l g) și închei (sfarsit). Colonie 1 punct, oraș 2, cel mai lung drum (≥5) 2; primul la 10 câștigă. Fără cărți de dezvoltare, porturi sau schimb între jucători. Informație ascunsă: vezi doar mâna ta.

## Scop

Primul la 10 puncte câștigă (la rândul lui).

## Pregătire

Tablă de 19 hexagoane (pădure `l`, argilă `a`, pășune `p`, lan `g`, munte `m`, un deșert) cu numere 2–12 amestecate la fiecare joc; 54 de vârfuri `vN`, 72 de muchii `eN`, hexagoane `hN`. Început: J1, J2, J2, J1 pun câte o colonie (`colonie v12`) și un drum lipit (`drum e7`); a doua colonie dă câte o resursă din fiecare hexagon vecin.

## Desfășurarea turei

`arunc` (două zaruri; hexagoanele cu suma dau resurse: 1 pe colonie, 2 pe oraș). La 7: cine are peste 7 cărți aruncă jumătate (automat, la întâmplare), apoi muți tâlharul (`bandit h5`) și furi o carte la întâmplare. Apoi oricâte construcții/schimburi și `sfarsit`.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `arunc`, `sfarsit`.
* `colonie v12`, `drum e7`, `oras v12`.
* `bandit h5`.
* `schimb l g` (4 cărți dintr-o resursă pentru 1 din alta).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (54): `colonie v0`, `colonie v1`, `colonie v2`, `colonie v3`, `colonie v4`, `colonie v5`, `colonie v6`, `colonie v7`, `colonie v8`, `colonie v9` …. Introduci `colonie v0`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (3): `drum e0`, `drum e5`, `drum e7`. Introduci `drum e5`; răspunsul calculatorului (în ordine): `colonie v33`; `drum e42`; `colonie v30`; `drum e36`.
3. Mutări legale (42): `colonie v2`, `colonie v3`, `colonie v4`, `colonie v6`, `colonie v8`, `colonie v9`, `colonie v10`, `colonie v11`, `colonie v12`, `colonie v13` …. Introduci `colonie v53`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Drum: l + a. Colonie: l a p g, pe vârf liber la cel puțin două muchii de orice construcție, legat de drumurile tale. Oraș: 2 g + 3 m, pe colonia ta.
* Piese: 5 colonii, 4 orașe, 15 drumuri. Hexagonul cu tâlhar nu produce.

## Punctaj

Colonie 1, oraș 2, cel mai lung drum (cel puțin 5 și strict mai lung decât al adversarului) 2.

## Sfârșitul jocului

Primul la 10 puncte. După 1500 de mutări câștigă cine are mai multe puncte (egal = remiză).

## Note despre AI

Informație ascunsă (mâna adversarului); zarurile sunt aruncate automat de joc. AI-ul nu se uită la cărțile adversarului. Tabla este amestecată la fiecare joc (reproductibil cu `--seed`).

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul NU este în `HOTSEAT_SAFE`: `--you both` afișează avertismentul „nu ascunde informația privată în modul la masă” (afișajul poate arăta mâna/informația Jucătorului 1), iar `hint` este dezactivat.

## Diferențe față de regulile tradiționale

* Fără cărți de dezvoltare, fără porturi, fără schimburi între jucători (doar 4:1 cu banca), fără limită de resurse în bancă.
