# Skitgubbe (`skitgubbe`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play skitgubbe --seed 1`. Sursa implementării: `polytabletop-games/src/skitgubbe.rs`.

> Rezumat din `polytabletop list`: Joc suedez de descărcare în două faze. Faza 1: cu pachet, joci o carte, adversarul trebuie să o bată cu una mai mare din aceeași culoare sau ia grămada; completezi la trei cărți. Faza 2: se joacă mâinile rămase, fără pachet; cine rămâne ultimul cu cărți e Skitgubbe (pierzătorul). Mutări: cartea, ridic (ia grămada), risc (riști o carte din pachet). Mâini și pachet ascunse.

## Reguli implementate

Joc suedez de descărcare în două faze. Faza 1: cu pachet, joci o carte, adversarul trebuie să o bată cu una mai mare din aceeași culoare sau ia grămada; completezi la trei cărți. Faza 2: se joacă mâinile rămase, fără pachet; cine rămâne ultimul cu cărți e Skitgubbe (pierzătorul). Mutări: cartea, ridic (ia grămada), risc (riști o carte din pachet). Mâini și pachet ascunse.

## Variante alese și limite

Adaptare pentru doi jucători a jocului suedez; AI-ul nu folosește `risc` (riscul îl lasă jucătorului uman).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: `set_viewer` este respectat (mod hot-seat sigur).

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
