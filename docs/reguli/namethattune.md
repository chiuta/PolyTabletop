# Ghicește melodia (`namethattune`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play namethattune --seed 1`. Sursa implementării: `polytabletop-games/src/namethattune.rs`.

> Rezumat din `polytabletop list`: Șase melodii din domeniul public date ca nume de note. Licitezi de câte note ai nevoie (ofer N); adversarul coboară oferta sau cere dovada (ghicesc). Cel mai mic licitator aude doar N note și alege titlul dintre 4 variante (o1–o4), apoi arat. Corect: +1 pentru el, greșit: +1 pentru celălalt. Mai multe puncte câștigă. Titlul corect rămâne ascuns până la arat.

## Reguli implementate

Șase melodii din domeniul public date ca nume de note. Licitezi de câte note ai nevoie (ofer N); adversarul coboară oferta sau cere dovada (ghicesc). Cel mai mic licitator aude doar N note și alege titlul dintre 4 variante (o1–o4), apoi arat. Corect: +1 pentru el, greșit: +1 pentru celălalt. Mai multe puncte câștigă. Titlul corect rămâne ascuns până la arat.

## Variante alese și limite

Melodiile sunt date ca șiruri de note (fără sunet); AI-ul le recunoaște comparând notele auzite cu repertoriul propriu, nu citește răspunsul.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: `set_viewer` este respectat (mod hot-seat sigur).

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
