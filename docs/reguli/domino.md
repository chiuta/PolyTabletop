# Domino (dublu-6) (`domino`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play domino --seed 1`. Sursa implementării: `polytabletop-games/src/domino.rs`.

> Rezumat din `polytabletop list`: 28 de piese, 7 fiecare, restul stoc. Începe cel mai mare dublu. Așază la stânga (S) sau dreapta (D); fără piesă pescuiești (pesc), cu stocul gol dai pas. Câștigă cine termină piesele (ia punctele adversarului) sau, la blocaj, cine are mai puține puncte. Mutări: 3-4:S, 3-4:D, pesc, pas.

## Scop

Rămâi fără piese sau, la blocaj, ai mai puține puncte în mână.

## Pregătire

Set dublu-6 de 28 de piese; fiecare primește 7, celelalte 14 formează stocul. Începe cine are cel mai mare dublu (altfel cea mai mare piesă după sumă), iar prima mutare este obligatoriu acea piesă.

## Desfășurarea turei

Așezi o piesă la un capăt al lanțului (`S` stânga, `D` dreapta) cu numărul potrivit. Fără piesă potrivită, pescuiești (`pesc`, câte una, până poți juca). Cu stocul gol și fără piesă spui `pas`.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Piesă + capăt: `3-4:S`, `3-4:D`; piesa se scrie cu numărul mic întâi. Prima piesă se scrie doar `6-6` (sau piesa de start).
* `pesc`, `pas`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `6-6`. Introduci `6-6`; răspunsul calculatorului (în ordine): `2-6:D`.
2. Mutări legale (2): `2-3:D`, `0-6:S`. Introduci `0-6:S`; răspunsul calculatorului (în ordine): `2-5:D`.
3. Mutări legale (2): `0-1:S`, `3-5:D`. Introduci `3-5:D`; răspunsul calculatorului (în ordine): `3-6:D`.

## Reguli speciale și variante alese

Nicio regulă specială dincolo de cele descrise mai sus.

## Punctaj

Câștigătorul rundei ia suma punctelor rămase în mâna adversarului (la blocaj, diferența).

## Sfârșitul jocului

Când cineva rămâne fără piese (victorie) sau la blocaj (doi pași la rând): câștigă cine are mai puține puncte în mână; egalitate = remiză.

## Note despre AI

Informație ascunsă (stocul și mâna adversarului). AI alpha-beta de adâncime 1 pe informația vizibilă.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Variantă cu stoc, doi jucători, o singură rundă.
