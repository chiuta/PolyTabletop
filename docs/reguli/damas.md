# Dame braziliene (Damas) (`damas`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play damas --seed 1`. Sursa implementării: `polytabletop-games/src/damas.rs`.

> Rezumat din `polytabletop list`: Dame internaționale pe 8×8: piesele simple mută înainte și capturează în toate direcțiile, damele zboară, captura este obligatorie și trebuie să fie cea mai mare. Remiză după 20 de mutări pe parte fără captură sau mutare de piesă simplă. Mutări: c3-d4, captură c3xe5xg7.

## Scop

Capturezi toate piesele adversarului sau îl lași fără mutări.

## Pregătire

Tablă 8×8, câte 12 piese pe pătratele întunecate; J1 (jos) mută primul.

## Desfășurarea turei

Muți o piesă sau capturezi; captura este obligatorie și trebuie să fie cea cu cele mai multe piese capturate. Piesele capturate se scot după întreaga mutare; o piesă care doar trece pe ultimul rând nu devine damă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `c3-d4`
* `c3xe5xg7`

## Simplificări și note

Remiză după 40 de semi-mutări fără captură și fără mutare de piesă simplă. AI: căutare alfa-beta.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
