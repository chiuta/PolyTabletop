# Dara (Nigeria) (`dara`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play dara --seed 1`. Sursa implementării: `polytabletop-games/src/dara.rs`.

> Rezumat din `polytabletop list`: Tablă 5×6, 12 piese pe jucător. Faza 1: plasare pe rând (fără capturi). Faza 2: mutare ortogonală cu un pas; o moară de exact 3 (rând sau coloană, nu 4) scoate o piesă adversă oarecare. Pierzi cu sub 3 piese sau blocat; remiză după 100 de mutări fără captură. Mutări: b3, b3-b4, b3-b4xd2 (fișiere a–f, rânduri 1–5).

## Scop

Lași adversarul cu mai puțin de 3 piese sau fără mutări.

## Pregătire

Tablă 5 rânduri × 6 coloane (fișiere a–f, rânduri 1–5), 12 piese pe jucător.

## Desfășurarea turei

Faza 1: plasare alternativă (fără capturi). Faza 2: mutări ortogonale cu un pas; o moară de exact trei piese (rând sau coloană; patru în linie nu contează) formată prin mutare scoate o piesă adversă oarecare.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Plasare: `b3`.
* Mutare: `b3-b4`.
* Mutare cu captură: `b3-b4xd2`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (30): `a1`, `b1`, `c1`, `d1`, `e1`, `f1`, `a2`, `b2`, `c2`, `d2` …. Introduci `a1`; răspunsul calculatorului (în ordine): `c3`.
2. Mutări legale (28): `b1`, `c1`, `d1`, `e1`, `f1`, `a2`, `b2`, `c2`, `d2`, `e2` …. Introduci `e3`; răspunsul calculatorului (în ordine): `b2`.
3. Mutări legale (26): `b1`, `c1`, `d1`, `e1`, `f1`, `a2`, `c2`, `d2`, `e2`, `f2` …. Introduci `f5`; răspunsul calculatorului (în ordine): `e2`.

## Reguli speciale și variante alese

* Se poate scoate orice piesă adversă, chiar și una din moară (varianta bead.game).

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Pierde cine rămâne cu mai puțin de 3 piese sau nu mai poate muta. Remiză după 100 de mutări consecutive fără captură.

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă (3, respectiv 4 în finaluri).

Stil AI în cod: alpha-beta cu adâncime fixă 3; alpha-beta cu adâncime fixă 4.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Wikipedia și alte surse interzic capturarea unei piese aflate într-o linie de trei a adversarului; codul permite orice piesă (varianta bead.game).
* Remiza după 100 de mutări fără captură este o regulă proprie.
