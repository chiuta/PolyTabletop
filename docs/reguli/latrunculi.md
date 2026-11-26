# Latrunculi (`latrunculi`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play latrunculi --seed 1`. Sursa implementării: `polytabletop-games/src/latrunculi.rs`.

> Rezumat din `polytabletop list`: Joc roman de masă, reconstituire modernă pe 8×8: 16 piese fiecare, toate mută ca turnul, captură în clește de către piesa mutată; câștigi lăsând adversarul cu o piesă sau fără mutări. Mutări: a2-a5.

## Scop

Lași adversarul cu o singură piesă sau fără mutări.

## Pregătire

8×8, câte 16 piese pe primele două rânduri (J1 jos, J2 sus).

## Desfășurarea turei

Muți o piesă ca turnul; dacă mutarea ta închide o piesă adversă între două ale tale pe o linie sau coloană, o capturezi. Intrarea voluntară între două piese adverse e sigură; în colț piesa cade când ambii vecini sunt adverși.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `a2-a5`

## Simplificări și note

Reconstituire modernă (regulile originale nu s-au păstrat); fără dux și fără blocarea piesei izolate. După 240 de semi-mutări câștigă cine are mai multe piese.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
