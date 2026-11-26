# Chaturaji (`chaturaji`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play chaturaji --seed 1`. Sursa implementării: `polytabletop-games/src/chaturaji.rs`.

> Rezumat din `polytabletop list`: Șahul indian cu patru armate, adaptat pentru doi (fiecare conduce două armate aliate). Arunci două zaruri, fiecare joacă un tip de piesă: 1/5 pion sau rege, 2 barcă, 3 cal, 4/6 elefant. Capturezi ambii regi adverși. Mutări: arunc, a2a3, pas.

## Scop

Capturezi ambii regi adverși sau ai mai multe puncte după 400 de semi-mutări.

## Pregătire

Tablă 8×8, patru armate (rege, elefant, cal, barcă, 4 pioni) în cele patru colțuri: J1 = Sud + Nord, J2 = Est + Vest.

## Desfășurarea turei

`arunc` (mutare separată, două zaruri); fiecare zar se joacă pe rând cu o piesă potrivită dintr-o armată a ta sau `pas`. Capturile dau 1/2/3/4/5 puncte (pion/barcă/cal/elefant/rege).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `arunc`
* `a2a3`
* `pas`

## Simplificări și note

Adaptare pentru doi, fără „triumful bărcilor” și fără tronul (sinhasana). Nu există șah.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
