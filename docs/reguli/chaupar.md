# Chaupar (cauri, 2 jucători) (`chaupar`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play chaupar --seed 1`. Sursa implementării: `polytabletop-games/src/chaupar.rs`.

> Rezumat din `polytabletop list`: 4 piese, circuit de 48 de pătrate (start 0 / 24), 4 cauri: 1–4 = pași, 0 = 8; 4 și 8 dau încă o aruncare; scoți din curte cu 1 sau 8 (intra). Castelele (multipli de 6) sunt sigure; captura trimite în curte și dă o aruncare. Coloană de acasă și centru exact. Mutări: arunc, intra, mut:12, mut:casa2, pas.

## Scop

Aduci primul toate cele 4 piese în centru.

## Pregătire

Fiecare jucător are 4 piese în curte. Circuit de 48 de pătrate (0–47): J1 pornește de la 0, J2 de la 24. După un tur întreg piesa urcă pe „coloana de acasă” (6 pătrate) și ajunge în centru exact (progres 54).

## Desfășurarea turei

Arunci 4 cauri: numărul de cauri cu gura în sus = pașii (1–4), niciunul în sus valorează 8. 4 și 8 dau încă o aruncare. O piesă din curte intră doar cu 1 sau 8. Dacă nicio piesă nu poate muta, mutarea forțată este `pas`.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `arunc`, `intra`.
* `mut:12` (piesa de pe pătratul 12), `mut:casa2` (piesa de pe coloana de acasă, poziția 2).
* `pas`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1): `intra`. Introduci `intra`; răspunsul calculatorului (în ordine): `arunc`; `intra`.
3. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Pătratele 0, 6, 12, … 42 sunt „castele” sigure.
* Aterizarea pe un pătrat obișnuit cu piese adverse le trimite în curte și dă o aruncare în plus (nu și pe coloana de acasă).

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Câștigă primul care aduce toate cele 4 piese în centru.

## Note despre AI

Zarurile (cauri) sunt informație publică, aruncate în stare. AI-ul (adâncime 1) nu vede aruncările viitoare.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Jocul real folosește 7 cauri, 8 pătrate „floare” sigure, cere o captură înainte de intrarea acasă și intrarea doar cu aruncări mari; varianta de aici (4 cauri, intrare cu 1/8, pătrate multiplu de 6) este o simplificare apropiată de Pachisi.
