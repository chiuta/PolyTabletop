# Artificii (cooperativ, partener AI) (`artificii`)

Categorie: Cooperativ. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play artificii --seed 1`. Sursa implementării: `polytabletop-games/src/artificii.rs`.

> Rezumat din `polytabletop list`: Tu și AI-ul construiți 5 șiruri de artificii (R G V A W) de la 1 la 5; vezi mâna partenerului, nu pe a ta. Tură: juc N, arunc N (+1 indiciu) sau indiciu R / indiciu 3 (cost 1 din 8 jetoane). 3 fitile arse = pierdeți. Obiectiv: 20 de puncte (echipa reușește = „Câștigă Jucătorul 1”; altfel remiză). Încheierea e o mutare forțată (sfârșit).

## Scop

Împreună cu partenerul AI atingeți cel puțin 20 de puncte (suma șirurilor, maximum 25).

## Pregătire

Cinci șiruri, câte unul pe culoare (R roșu, G galben, V verde, A albastru, W alb), de la 1 la 5. Pachet de 50 de cărți (pe culoare: 1×3, 2×2, 3×2, 4×2, 5×1), 5 cărți în mână, 8 jetoane de indiciu, 3 fitile. Nu îți vezi propriile cărți, doar pe ale partenerului.

## Desfășurarea turei

Alegi: (a) joci o carte din mâna ta; dacă e următoarea din șirul culorii, urcă șirul, altfel arde o fitilă; (b) arunci o carte și recuperezi un jeton (nu cu 8); (c) dai un indiciu partenerului (o culoare sau un număr: arăți toate cărțile lui de acel fel, cel puțin una), cost 1 jeton. Cartea 5 jucată corect dă un jeton înapoi. După ce pachetul se golește, fiecare mai are o tură.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `juc N` (poziția N din mâna ta), `arunc N`.
* `indiciu R` (culoare), `indiciu 3` (număr).
* `sfârșit` (mutare forțată de încheiere).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (13): `juc 1`, `juc 2`, `juc 3`, `juc 4`, `juc 5`, `indiciu R`, `indiciu G`, `indiciu V`, `indiciu A`, `indiciu 1` …. Introduci `juc 1`; răspunsul calculatorului (în ordine): `indiciu 1`.
2. Mutări legale (18): `juc 1`, `juc 2`, `juc 3`, `juc 4`, `juc 5`, `arunc 1`, `arunc 2`, `arunc 3`, `arunc 4`, `arunc 5` …. Introduci `arunc 5`; răspunsul calculatorului (în ordine): `indiciu 1`.
3. Mutări legale (18): `juc 1`, `juc 2`, `juc 3`, `juc 4`, `juc 5`, `arunc 1`, `arunc 2`, `arunc 3`, `arunc 4`, `arunc 5` …. Introduci `indiciu 5`; răspunsul calculatorului (în ordine): `indiciu 4`.

## Reguli speciale și variante alese

* Trei fitile arse = pierdeți (scor 0).

## Punctaj

Suma șirurilor (maximum 25).

## Sfârșitul jocului

Echipa reușește (Jucătorul 1 „câștigă”) la cel puțin 20 de puncte; altfel remiză (obiectiv neatins).

## Note despre AI

Informație ascunsă: mâna ta nu apare; vezi mâna AI-ului și ce ți-a arătat prin indicii. AI-ul nu își citește cărțile: raționează din indicii primite și din cărțile vizibile. Încheierea e o mutare forțată, ca AI-ul să nu „vadă” dinainte dacă o carte jucată reușește.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Variantă originală cooperativă; rezultatul este „Câștigă Jucătorul 1” sau remiză, niciodată înfrângere în sensul adversar.
