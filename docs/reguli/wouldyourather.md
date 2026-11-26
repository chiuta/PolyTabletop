# Ai prefera… (`wouldyourather`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play wouldyourather --seed 1`. Sursa implementării: `polytabletop-games/src/wouldyourather.rs`.

> Rezumat din `polytabletop list`: Șase dileme. La fiecare, fiecare alege în secret răspunsul propriu (a sau b) și ghicește răspunsul celuilalt: mutări aa, ab, ba, bb (prima literă = răspunsul tău, a doua = ghicitul tău), apoi arat. Fiecare ghicit corect = 1 punct; mai multe puncte câștigă. Alegeri ascunse.

## Reguli implementate

Șase dileme. La fiecare, fiecare alege în secret răspunsul propriu (a sau b) și ghicește răspunsul celuilalt: mutări aa, ab, ba, bb (prima literă = răspunsul tău, a doua = ghicitul tău), apoi arat. Fiecare ghicit corect = 1 punct; mai multe puncte câștigă. Alegeri ascunse.

## Variante alese și limite

Dilemele sunt în română (bancă de 14, se aleg 6); punctul vine din ghicirea alegerii celuilalt.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: `set_viewer` este respectat (mod hot-seat sigur).

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
