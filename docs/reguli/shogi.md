# Shogi (șah japonez) (`shogi`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play shogi --seed 1`. Sursa implementării: `polytabletop-games/src/shogi.rs`.

> Rezumat din `polytabletop list`: 9×9; piesele capturate se pun înapoi (drop). Promovare în zona adversă (obligatorie dacă piesa n-ar mai avea mutări). Drop interzis: nifu (doi pioni pe fișier), pion/lance/cal fără mutări, pion care dă mat. Mat = pierdere; a 4-a repetiție sau 320 de semi-mutări = remiză. Mutări: 7g7f, 2b8h+ (promovare), P*5e (drop).

## Scop

Dai mat regelui adversar.

## Pregătire

Tablă 9×9. Jucătorul 1 = sente (negru, jos, rândurile g–i), Jucătorul 2 = gote (sus). Coordonate tradiționale: fișiere `1`–`9` (de la dreapta la stânga, văzute de sente), rânduri `a`–`i` (de sus în jos).

## Desfășurarea turei

Muți o piesă sau plasezi (drop) o piesă capturată din mâna ta.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Mutare: `7g7f`.
* Mutare cu promovare: `2b8h+`.
* Drop: `P*5e`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (30): `9g9f`, `8g8f`, `7g7f`, `6g6f`, `5g5f`, `4g4f`, `3g3f`, `2g2f`, `1g1f`, `2h1h` …. Introduci `7g7f`; răspunsul calculatorului (în ordine): `7a6b`.
2. Mutări legale (37): `8h3c`, `8h3c+`, `7f7e`, `9g9f`, `8g8f`, `6g6f`, `5g5f`, `4g4f`, `3g3f`, `2g2f` …. Introduci `2h5h`; răspunsul calculatorului (în ordine): `3c3d`.
3. Mutări legale (37): `8h2b`, `8h2b+`, `8h3c+`, `7f7e`, `9g9f`, `8g8f`, `6g6f`, `5g5f`, `4g4f`, `3g3f` …. Introduci `1i1h`; răspunsul calculatorului (în ordine): `2a3c`.

Notă: AI-ul acestui joc folosește un buget de timp sau simulări; răspunsurile calculatorului pot diferi de la o rulare la alta, chiar cu același `--seed`. Mutările legale și efectul mutărilor tale sunt deterministe.

## Reguli speciale și variante alese

* Piesele capturate trec în mâna capturantului și pot fi puse înapoi (drop).
* Drop-uri interzise: pe pătrat ocupat; pion/lance pe ultimul rând și cal pe ultimele două (n-ar mai avea mutări); doi pioni nepromovați pe același fișier (nifu); pion care dă mat imediat (uchifuzume).
* Promovarea e opțională în zona adversă și obligatorie când piesa n-ar mai avea mutări; capturată, piesa revine la forma de bază.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Mat = pierdere. A patra apariție a aceleiași poziții (sennichite) sau 320 de semi-mutări = remiză.

## Note despre AI

Informație completă. Alpha-beta cu buget de timp (500 ms, adâncime maximă 10).

Stil AI în cod: alpha-beta cu adâncire iterativă și buget de timp (500 ms, adâncime maximă 10).

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Repetiția = remiză după a patra apariție; șahul perpetuu nu se pedepsește.
* Limită de 320 de semi-mutări, după care partida e remiză.
