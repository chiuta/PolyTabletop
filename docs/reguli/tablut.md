# Tablut (`tablut`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tablut --seed 1`. Sursa implementării: `polytabletop-games/src/tablut.rs`.

> Rezumat din `polytabletop list`: Hnefatafl lapon pe 9×9: 16 atacatori (J1, primul) contra 8 apărători și un rege. Toate piesele mută ca turnul; captură în clește; regele scapă pe orice pătrat de margine, atacatorii îl capturează pe patru laturi. Mutări: e1-e3.

## Scop

Atacatori: capturezi regele. Apărători: duci regele pe orice pătrat de pe margine.

## Pregătire

Tablă 9×9; atacatorii (J1) pe margini, apărătorii și regele (J2) în centru, regele pe tron (e5).

## Desfășurarea turei

Muți o piesă ca turnul (oricât, fără salturi). Doar regele se oprește pe tron. O piesă adversă prinsă între două ale tale (sau între una a ta și tronul gol) este capturată.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `e1-e3`

## Simplificări și note

Remiză după 300 de semi-mutări; cine nu mai are mutări pierde. AI: alfa-beta.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
