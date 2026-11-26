# Xiangqi (șah chinezesc) (`xiangqi`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play xiangqi --seed 1`. Sursa implementării: `polytabletop-games/src/xiangqi.rs`.

> Rezumat din `polytabletop list`: 9×10, palat, râu, tun cu ecran, cal blocabil, generalii nu se pot privi. Mat sau pat = pierdere. Mutări: h2e2 (fișiere a–i, rânduri 0–9).

## Scop

Dai mat generalului adversar sau îl lași fără nicio mutare legală (mat sau pat = pierdere).

## Pregătire

Tablă 9×10. Jucătorul 1 = roșu (jos, rândurile 0–4), Jucătorul 2 = negru (sus). Roșu începe.

## Desfășurarea turei

Muți o piesă; nicio mutare nu poate lăsa propriul general atacat.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Coordonate în stil UCCI: fișiere `a`–`i`, rânduri `0`–`9`; de la, la, fără separator: `h2e2`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (44): `b2b9`, `h2h9`, `a0a1`, `a0a2`, `b0c2`, `b0a2`, `c0e2`, `c0a2`, `d0e1`, `e0e1` …. Introduci `h2e2`; răspunsul calculatorului (în ordine): `h7e7`.
2. Mutări legale (35): `b2b9`, `e2e6`, `a0a1`, `a0a2`, `b0c2`, `b0a2`, `c0a2`, `d0e1`, `e0e1`, `f0e1` …. Introduci `b2a2`; răspunsul calculatorului (în ordine): `e7e3`.
3. Mutări legale (10): `e2e6`, `d0e1`, `f0e1`, `e2f2`, `e2g2`, `e2h2`, `e2i2`, `e2d2`, `e2c2`, `e2b2`. Introduci `e2b2`; răspunsul calculatorului (în ordine): `b7e7`.

Notă: AI-ul acestui joc folosește un buget de timp sau simulări; răspunsurile calculatorului pot diferi de la o rulare la alta, chiar cu același `--seed`. Mutările legale și efectul mutărilor tale sunt deterministe.

## Reguli speciale și variante alese

* Generalul și consilierii rămân în palat; elefanții nu trec râul și sunt blocați la „ochi”; caii sunt blocați la „picior”; tunurile capturează sărind exact o piesă; soldații capătă pași laterali după râu.
* Cei doi generali nu se pot privi pe o coloană liberă.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Mat sau pat = pierdere. Remiză după 120 de semi-mutări fără captură (simplificare a regulilor de repetiție).

## Note despre AI

Informație completă. Alpha-beta cu buget de timp (500 ms, adâncime maximă 12).

Stil AI în cod: alpha-beta cu adâncire iterativă și buget de timp (500 ms, adâncime maximă 12).

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Remiza după 120 de semi-mutări fără captură înlocuiește regulile complexe de repetiție și șah perpetuu.
