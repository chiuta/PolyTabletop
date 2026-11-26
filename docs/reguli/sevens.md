# Șapte (Sevens) (`sevens`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sevens --seed 1`. Sursa implementării: `polytabletop-games/src/sevens.rs`.

> Rezumat din `polytabletop list`: Șapte în doi: câte 26 de cărți. Un 7 deschide culoarea la mijloc; apoi construiești în sus și în jos, cu o carte din aceeași culoare lipită de șir. Dacă nu poți juca, treci. Câștigă cine rămâne primul fără cărți. Mutări: cartea (ex. 7h) sau pas. Mâini ascunse.

## Reguli implementate

Șapte în doi: câte 26 de cărți. Un 7 deschide culoarea la mijloc; apoi construiești în sus și în jos, cu o carte din aceeași culoare lipită de șir. Dacă nu poți juca, treci. Câștigă cine rămâne primul fără cărți. Mutări: cartea (ex. 7h) sau pas. Mâini ascunse.

## Variante alese și limite

Adaptare pentru doi jucători (Fan Tan clasic se joacă în 3–8): se împart toate cele 52 de cărți (26 + 26); câștigă cine golește primul mâna.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: `set_viewer` este respectat (mod hot-seat sigur).

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
