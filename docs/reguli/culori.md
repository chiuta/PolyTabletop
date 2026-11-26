# Culori (potrivit cărți colorate) (`culori`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play culori --seed 1`. Sursa implementării: `polytabletop-games/src/culori.rs`.

> Rezumat din `polytabletop list`: Joc original: 108 cărți (4 culori cu cifre 0–9, sari, intoarce, plus2, plus joker și joker4). Joci o carte de aceeași culoare sau cu aceeași cifră/simbol; în doi, sari/intoarce = adversarul pierde rândul, plus2 = trage 2, joker4 = trage 4 (jokerii aleg culoarea: joker:r). Fără carte jucabilă tragi (trage), apoi o joci sau dai pas. Câștigă cine rămâne primul fără cărți. Informație ascunsă: vezi doar mâna ta. Mutări: r5, gsari, vplus2, joker:a, trage, pas.

## Scop

Rămâi primul fără cărți în mână (o singură rundă).

## Pregătire

Joc original cu 108 cărți: patru culori (r roșu, v verde, a albastru, g galben): cifra 0 o dată, cifrele 1–9 de două ori, câte 2 × `sari`, `intoarce`, `plus2`; plus 4 × `joker` și 4 × `joker4`. 7 cărți fiecare; prima carte de pe talon are cifră.

## Desfășurarea turei

Joci o carte de aceeași culoare (curentă) sau cu aceeași cifră/simbol ca ultima carte; jokerii merg oricând și aleg culoarea. Fără carte jucabilă tragi o carte (`trage`); dacă se potrivește o poți juca sau spui `pas`. Nu poți trage cât ai o carte jucabilă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Culoare + cifră/simbol: `r5`, `gsari`, `vplus2`.
* Joker cu culoarea aleasă: `joker:a`, `joker4:r`.
* `trage`, `pas`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (2): `rsari`, `r6`. Introduci `rsari`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1): `r6`. Introduci `r6`; răspunsul calculatorului (în ordine): `joker4:a`; `aplus2`; `rplus2`; `joker:a`.
3. Mutări legale (3): `aplus2`, `a5`, `asari`. Introduci `asari`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* În doi, `sari` și `intoarce` fac adversarul să piardă rândul (joci iar); `plus2` = adversarul trage 2 și pierde rândul; `joker4` la fel cu 4.
* Fără strigarea „ultima carte” și fără contestarea lui `joker4`.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Câștigă cine rămâne primul fără cărți.

## Note despre AI

Informație ascunsă; afișajul arată doar mâna ta. AI-ul se uită doar la mâna lui, la talon și la numărul cărților adversarului.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Joc original cu mecanică generică; nume, texte și pachet proprii.
* Simplificări: nu poți trage cât ai o carte jucabilă, fără „ultima carte”, o singură rundă.
