# Linii de acțiune (8×8) (`linii`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play linii --seed 1`. Sursa implementării: `polytabletop-games/src/linii.rs`.

> Rezumat din `polytabletop list`: Muți o piesă pe linie atâtea pătrate câte piese sunt pe acea linie; poți sări piese proprii, nu ale adversarului, și captezi aterizând pe una; câștigă cine își unește toate piesele într-un grup (remiză după 300 de mutări). Mutări: b1-b3, b1xd3.

## Scop

Îți unești toate piesele într-un singur grup (vecinătate pe 8 direcții).

## Pregătire

Tablă 8×8; X (J1) începe.

## Desfășurarea turei

Muți o piesă în linie dreaptă exact atâtea pătrate câte piese (ale ambilor jucători) se află pe linia respectivă (orizontală, verticală, diagonală). Poți sări piese proprii, nu ale adversarului, și capturezi aterizând pe o piesă adversă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Mutare simplă: `b1-b3`.
* Captură: `b1xd3`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (36): `b1-h1`, `b1-b3`, `b1-d3`, `c1-c3`, `c1-e3`, `c1xa3`, `d1-d3`, `d1-f3`, `d1-b3`, `e1-e3` …. Introduci `b1-h1`; răspunsul calculatorului (în ordine): `h7-g6`.
2. Mutări legale (34): `c1-c3`, `c1-e3`, `c1xa3`, `d1-d3`, `d1-f3`, `d1-b3`, `e1-e3`, `e1-g3`, `e1-c3`, `f1-f3` …. Introduci `b8-h8`; răspunsul calculatorului (în ordine): `h2-g3`.
3. Mutări legale (32): `c1-c3`, `c1-e3`, `c1xa3`, `d1-d3`, `d1-f3`, `d1-b3`, `e1-e3`, `e1-c3`, `f1-f3`, `f1xh3` …. Introduci `h8-g7`; răspunsul calculatorului (în ordine): `h3-f5`.

Notă: AI-ul acestui joc folosește un buget de timp sau simulări; răspunsurile calculatorului pot diferi de la o rulare la alta, chiar cu același `--seed`. Mutările legale și efectul mutărilor tale sunt deterministe.

## Reguli speciale și variante alese

Nicio regulă specială dincolo de cele descrise mai sus.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Cine își unește primul toate piesele câștigă; după 300 de mutări jocul este remiză.

## Note despre AI

Informație completă. Alpha-beta cu buget de timp (600 ms, adâncime maximă 6).

Stil AI în cod: alpha-beta cu adâncire iterativă și buget de timp (600 ms, adâncime maximă 6).

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

Sursa nu documentează simplificări; regulile urmează varianta standard descrisă mai sus.
