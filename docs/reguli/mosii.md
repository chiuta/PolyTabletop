# Moșii (afaceri imobiliare cu zaruri) (`mosii`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mosii --seed 1`. Sursa implementării: `polytabletop-games/src/mosii.rs`.

> Rezumat din `polytabletop list`: Joc original, 32 de câmpuri: arunci două zaruri (automat), cumperi terenuri și gări (cumpar / las), încasezi chirii (grup complet = chirie dublă, case: construiesc 3), taxe, evenimente Noroc și închisoare (platesc 50 sau dublă); dublă = încă o aruncare. Faliment = pierdere; după 1500 de mutări câștigă averea mai mare. Fără licitații și ipoteci.

## Scop

Ruinezi adversarul; la limita de 1500 de mutări câștigă cine are averea mai mare.

## Pregătire

Tablă cu 32 de câmpuri; 1000 de lei fiecare; pornești de la Start și la fiecare trecere prin Start (sau aterizare pe el) încasezi 150.

## Desfășurarea turei

Arunci (`arunc`, două zaruri aruncate automat), muți, apoi, după câmp: cumperi sau lași o proprietate liberă, plătești chirie, taxe, eveniment Noroc. Poți construi case.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `arunc`.
* `cumpar` / `las` (proprietate liberă pe care ai aterizat și ți-o permiți).
* `construiesc N` (casă pe câmpul N), `platesc` (părăsești închisoarea contra 50).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): `arunc`; `cumpar`; `arunc`.
2. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (2): `cumpar`, `las`. Introduci `las`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Chirie = a zecea parte din preț; cu grupul complet se dublează; cu 1/2/3 case: ×4 / ×10 / ×20. Gările: 20/40/80/160 după câte ai.
* Casele se construiesc doar cu grupul complet, uniform; costul unei case e fixat pe grup.
* Dubla = încă o aruncare; a treia dublă la rând = închisoare. Din închisoare ieși cu `platesc` (50), cu o dublă la `arunc` sau automat plătind după a treia încercare ratată.
* Dacă nu poți plăti, ți se vând casele (la jumătate din cost); dacă tot nu ajunge, dai faliment.

## Punctaj

Averea = bani + proprietăți + case.

## Sfârșitul jocului

Faliment = pierdere; după 1500 de mutări câștigă averea mai mare (egal = remiză).

## Note despre AI

Zarurile sunt aruncate automat în starea jocului (informație publică). AI-ul (adâncime 1) nu vede aruncările viitoare.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Fără licitații și fără ipoteci; tablă, prețuri și evenimente originale.
