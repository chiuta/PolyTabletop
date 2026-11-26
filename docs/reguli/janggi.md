# Janggi (șah coreean) (`janggi`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play janggi --seed 1`. Sursa implementării: `polytabletop-games/src/janggi.rs`.

> Rezumat din `polytabletop list`: 9×10, palate cu diagonale, Cho (jos) începe. Car pe diagonalele palatului, tun care sare exact o piesă (nu tun), cal blocabil, elefant (1 ortogonal + 2 diagonale) blocabil, soldați înainte/lateral. Pass permis când nu ești în șah; mat = pierdere; repetiție, 2 pass-uri sau 150 de semi-mutări fără captură = remiză. Mutări: b2e2, pass.

## Scop

Dai mat generalului adversar.

## Pregătire

Tablă 9×10, palate cu diagonale. Jucătorul 1 = Cho (jos, rândurile 0–4, mută primul), Jucătorul 2 = Han (sus). Coordonate ca la xiangqi.

## Desfășurarea turei

Muți o piesă sau treci (`pass`, permis doar când nu ești în șah).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* De la, la: `b2e2`.
* `pass`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (32): `a0a1`, `a0a2`, `b0d3`, `c0d2`, `d0e0`, `d0d1`, `f0e0`, `f0f1`, `h0i2`, `h0g2` …. Introduci `a0a1`; răspunsul calculatorului (în ordine): `g9f7`.
2. Mutări legale (35): `b0d3`, `c0d2`, `d0e0`, `d0d1`, `f0e0`, `f0f1`, `h0i2`, `h0g2`, `i0i1`, `i0i2` …. Introduci `e1e2`; răspunsul calculatorului (în ordine): `h7e7`.
3. Mutări legale (41): `b0d3`, `c0d2`, `d0e0`, `d0d1`, `d0e1`, `f0e0`, `f0f1`, `f0e1`, `h0i2`, `h0g2` …. Introduci `pass`; răspunsul calculatorului (în ordine): `e6d6`.

Notă: AI-ul acestui joc folosește un buget de timp sau simulări; răspunsurile calculatorului pot diferi de la o rulare la alta, chiar cu același `--seed`. Mutările legale și efectul mutărilor tale sunt deterministe.

## Reguli speciale și variante alese

* Piese: General (în palat), Gardă (Sa, în palat), Elefant (Sang: un pas ortogonal + două diagonale, blocabil), Cal (Ma: un pas ortogonal + unul diagonal, blocabil), Car (Cha: turn; în palat și pe diagonale), Tun (Po: sare exact peste o piesă, nu peste/pe tun), Soldat (înainte sau lateral, pe diagonalele palatului inamic înainte).
* Generalii nu se pot privi pe o coloană liberă (mutarea e ilegală).

## Punctaj

Fără punctaj (fără numărătoare de puncte).

## Sfârșitul jocului

Mat = pierdere. Remiză: două pass-uri consecutive, o poziție repetată de 3 ori sau 150 de semi-mutări fără captură.

## Note despre AI

Informație completă. Alpha-beta cu buget de timp (500 ms, adâncime maximă 12).

Stil AI în cod: alpha-beta cu adâncire iterativă și buget de timp (500 ms, adâncime maximă 12).

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* În regulile reale „bikjang” (generalii față în față) NU e o mutare ilegală: cine e la mutare trebuie să rupă poziția, altfel jocul e remiză; aici mutarea e ilegală.
* Turneele coreene nu acceptă remiza prin repetiție și decid după numărătoarea de puncte (car 13, tun 7, cal 5, elefant 3, gardă 3, soldat 2; Han primește 1,5); aici repetarea de 3 ori e remiză și nu există numărătoare.
