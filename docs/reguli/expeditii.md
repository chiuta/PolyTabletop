# Expediții (`expeditii`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play expeditii --seed 1`. Sursa implementării: `polytabletop-games/src/expeditii.rs`.

> Rezumat din `polytabletop list`: Cinci masive (B F R D C), cărți 2–10 și 3 angajamente (x) pe culoare; mână de 8. Tură: joci o carte pe expediția ta (valori crescătoare) sau o arunci (juc B5, arunc B5), apoi tragi din pachet sau iei de pe o grămadă (trag, ia F). Scor pe expediție: (suma − 20) × (1 + angajamente), +20 cu ≥8 cărți. Se termină la ultima carte din pachet.

## Scop

Ai cel mai mare scor total pe cele cinci expediții.

## Pregătire

Cinci masive (B Bucegi, F Făgăraș, R Retezat, D Rodna, C Ceahlău), fiecare cu cărți 2–10 și trei cărți „angajament” (`x`). Pachet de 60; 8 cărți în mână.

## Desfășurarea turei

(1) Joci o carte pe expediția ta (valori strict crescătoare; angajamentele doar înaintea numerelor) sau o arunci pe grămada culorii ei; (2) tragi din pachet sau iei vârful unei grămezi de aruncate (nu pe cea pe care tocmai ai aruncat).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `juc B5` (joci), `arunc B5` (arunci).
* `trag` (din pachet), `ia F` (vârful grămezii F).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (16): `juc B10`, `arunc B10`, `juc F4`, `arunc F4`, `juc Rx`, `arunc Rx`, `juc R7`, `arunc R7`, `juc Dx`, `arunc Dx` …. Introduci `juc B10`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1): `trag`. Introduci `trag`; răspunsul calculatorului (în ordine): `arunc Rx`; `trag`.
3. Mutări legale (14): `juc F4`, `arunc F4`, `juc Rx`, `arunc Rx`, `juc R7`, `arunc R7`, `juc Dx`, `arunc Dx`, `juc C5`, `arunc C5` …. Introduci `arunc C10`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

Nicio regulă specială dincolo de cele descrise mai sus.

## Punctaj

Pe fiecare expediție începută: (suma − 20) × (1 + angajamente), +20 dacă are cel puțin 8 cărți.

## Sfârșitul jocului

Jocul se termină când se trage ultima carte din pachet.

## Note despre AI

Informație ascunsă: vezi doar mâna ta. AI-ul evaluează doar ce vede (nu citește pachetul sau mâna adversarului; cartea trasă din pachet nu intră în evaluare).

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Variantă originală cu mecanică de „expediții în culori”.
