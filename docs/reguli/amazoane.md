# Amazoane (10×10) (`amazoane`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play amazoane --seed 1`. Sursa implementării: `polytabletop-games/src/amazoane.rs`.

> Rezumat din `polytabletop list`: Patru amazoane pe parte; muți una ca o regină de șah, apoi tragi o săgeată (tot ca o regină) care blochează pătratul; pierde cine nu mai poate muta. Mutări: d1-d5/d8 (de la–la/săgeată).

## Scop

Lași adversarul fără nicio mutare legală (pierde cel care nu mai poate muta).

## Pregătire

Tablă 10×10, patru amazoane pe jucător. J1 (X) începe.

## Desfășurarea turei

Muți una dintre amazoanele tale ca o regină de șah (fără a sări), apoi tragi o săgeată dintr-un pătrat în care ai ajuns, tot ca o regină; pătratul săgeții este blocat definitiv.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `de-la` `-` `la` `/` `săgeată`, scris fără spații: `d1-d5/d8` înseamnă amazoana de pe d1 merge pe d5 și trage săgeata pe d8.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (2176): `d1-c1/b1`, `d1-c1/a1`, `d1-c1/d1`, `d1-c1/e1`, `d1-c1/f1`, `d1-c1/b2`, `d1-c1/a3`, `d1-c1/c2`, `d1-c1/c3`, `d1-c1/c4` …. Introduci `d1-c1/b1`; răspunsul calculatorului (în ordine): `g10-g2/c6`.
2. Mutări legale (1585): `c1-d1/c1`, `c1-d1/e1`, `c1-d1/f1`, `c1-d1/c2`, `c1-d1/b3`, `c1-d1/d2`, `c1-d1/d3`, `c1-d1/d4`, `c1-d1/d5`, `c1-d1/d6` …. Introduci `a4-d1/g4`; răspunsul calculatorului (în ordine): `a7-e3/d2`.
3. Mutări legale (669): `c1-b2/a1`, `c1-b2/c1`, `c1-b2/a2`, `c1-b2/c2`, `c1-b2/a3`, `c1-b2/b3`, `c1-b2/b4`, `c1-b2/b5`, `c1-b2/b6`, `c1-b2/b7` …. Introduci `j4-j6/f10`; răspunsul calculatorului (în ordine): `g2-j5/i6`.

Notă: AI-ul acestui joc folosește un buget de timp sau simulări; răspunsurile calculatorului pot diferi de la o rulare la alta, chiar cu același `--seed`. Mutările legale și efectul mutărilor tale sunt deterministe.

## Reguli speciale și variante alese

* Săgețile blochează pătrate pentru amândoi, pentru tot restul jocului.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Cine nu mai are nicio mutare (mutare + săgeată) pierde.

## Note despre AI

Informație completă. Alpha-beta cu buget de timp (800 ms, adâncime maximă 3).

Stil AI în cod: alpha-beta cu adâncire iterativă și buget de timp (800 ms, adâncime maximă 3).

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

Sursa nu documentează simplificări; regulile urmează varianta standard descrisă mai sus.
