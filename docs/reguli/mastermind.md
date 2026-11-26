# Spargerea codului (cu schimb de roluri) (`mastermind`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mastermind --seed 1`. Sursa implementării: `polytabletop-games/src/mastermind.rs`.

> Rezumat din `polytabletop list`: Cod de 4 cifre 1–6 (repetiții permise), cel mult 10 încercări; răspuns N = poziție exactă, A = culoare corectă pe altă poziție. Runda 1: spargi codul ascuns al AI-ului. Runda 2: îți alegi un cod (cod:1234) și AI-ul îl sparge. Câștigă cine a avut nevoie de mai puține încercări (nespart = 11). Mutări: 1234, cod:1234, confirm.

## Scop

Spargi codul în mai puține încercări decât adversarul (nespart = 11).

## Pregătire

Cod de 4 cifre 1–6, repetiții permise, cel mult 10 încercări. Runda 1: spargi codul ascuns al AI-ului. Runda 2: îți alegi un cod, iar AI-ul îl sparge.

## Desfășurarea turei

Încercare = patru cifre. Răspuns: N = câte poziții exacte, A = câte culori corecte pe poziții greșite.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Încercare: `1123`.
* Cod secret (runda 2): `cod:1234`.
* `confirm` (forțată: codul-maestru confirmă după încercarea reușită sau a 10-a).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1296): `1111`, `1112`, `1113`, `1114`, `1115`, `1116`, `1121`, `1122`, `1123`, `1124` …. Introduci `1111`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1296): `1111`, `1112`, `1113`, `1114`, `1115`, `1116`, `1121`, `1122`, `1123`, `1124` …. Introduci `4111`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (1296): `1111`, `1112`, `1113`, `1114`, `1115`, `1116`, `1121`, `1122`, `1123`, `1124` …. Introduci `6666`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Răspunsurile sunt calculate de joc, nu de om.

## Punctaj

Numărul de încercări (nespart = 11).

## Sfârșitul jocului

După ambele runde câștigă cine a avut nevoie de mai puține încercări; egal = remiză.

## Note despre AI

Codul AI-ului este generat cu generatorul jocului (reproductibil cu `--seed`). AI-ul spărgător calculează scorul fiecărei încercări înainte de a o aplica, doar din încercările lui anterioare și răspunsurile lor: o încercare compatibilă cu toate răspunsurile de până acum primește scor maxim. `confirm` este forțată ca AI-ul să nu vadă dinainte dacă ultima încercare e corectă.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Două runde cu schimb de roluri, într-un singur joc.
