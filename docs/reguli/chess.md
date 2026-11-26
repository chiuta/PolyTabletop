# Șah (`chess`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play chess --seed 1`. Sursa implementării: `polytabletop-games/src/chess.rs`.

> Rezumat din `polytabletop list`: Reguli complete: rocadă (mută regele două pătrate: e1g1 sau e1c1), en passant, promovare (obligatorie; e7e8q, sufixul q/r/b/n alege piesa), remiză prin pat, repetiție de trei ori, 50 de mutări fără captură sau pion, material insuficient. Mutări: e2e4, e7e8q.

## Scop

Dai mat regelui adversar: regele lui este atacat și nu există nicio mutare legală care să scoată atacul.

## Pregătire

Tablă 8×8, poziția standard de start. Jucătorul 1 are albele (jos) și mută primul; Jucătorul 2 are negrele.

## Desfășurarea turei

Fiecare jucător mută o singură piesă pe tură. O mutare care lasă propriul rege în șah este ilegală.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Notație de coordonate: pătratul de plecare urmat de cel de sosire, fără separator: `e2e4`.
* Promovarea este obligatorie și se scrie cu un sufix: `q` (damă), `r` (turn), `b` (nebun), `n` (cal): `e7e8q`.
* Rocada se scrie ca mutarea regelui cu două pătrate: `e1g1` (mica) sau `e1c1` (marea).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (20): `a2a3`, `a2a4`, `b2b3`, `b2b4`, `c2c3`, `c2c4`, `d2d3`, `d2d4`, `e2e3`, `e2e4` …. Introduci `e2e4`; răspunsul calculatorului (în ordine): `g7g5`.
2. Mutări legale (30): `a2a3`, `a2a4`, `b2b3`, `b2b4`, `c2c3`, `c2c4`, `d2d3`, `d2d4`, `f2f3`, `f2f4` …. Introduci `g1f3`; răspunsul calculatorului (în ordine): `g5g4`.
3. Mutări legale (27): `a2a3`, `a2a4`, `b2b3`, `b2b4`, `c2c3`, `c2c4`, `d2d3`, `d2d4`, `g2g3`, `h2h3` …. Introduci `f1c4`; răspunsul calculatorului (în ordine): `g4f3`.

Notă: AI-ul acestui joc folosește un buget de timp sau simulări; răspunsurile calculatorului pot diferi de la o rulare la alta, chiar cu același `--seed`. Mutările legale și efectul mutărilor tale sunt deterministe.

## Reguli speciale și variante alese

* Rocadă, en passant și promovare (obligatorie) sunt implementate complet.
* Pat, repetiție de trei ori a poziției, 50 de mutări fără captură sau mutare de pion și material insuficient dau remiză.

## Punctaj

Nu există punctaj: rezultatul este victorie, înfrângere sau remiză.

## Sfârșitul jocului

Mat (victorie); pat, repetiție de trei ori a poziției, 50 de mutări (100 de semi-mutări) fără captură sau mutare de pion și material insuficient (remiză).

## Note despre AI

Informație completă, fără elemente aleatoare. AI-ul este alpha-beta cu adâncire iterativă și buget de timp (500 ms, adâncime maximă 14).

Stil AI în cod: alpha-beta cu adâncire iterativă și buget de timp (500 ms, adâncime maximă 14); alpha-beta cu adâncime configurabilă.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

Sursa nu documentează simplificări; regulile urmează varianta standard descrisă mai sus.
