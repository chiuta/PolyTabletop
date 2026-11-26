# Peg solitaire (duel) (`pegsol`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pegsol --seed 1`. Sursa implementării: `polytabletop-games/src/pegsol.rs`.

> Rezumat din `polytabletop list`: Tabla engleză cu 33 de găuri, 32 de bile, centrul liber. O mutare: o bilă sare ortogonal peste o vecină într-o gaură liberă și bila sărită se scoate. Duel: fiecare joacă propria tablă identică, pe rând; cine rămâne fără mutări iese din joc. La final câștigă cine are mai puține bile (egal: ultima bilă în centru). Mutări: d2-d4.

## Reguli implementate

Tabla engleză cu 33 de găuri, 32 de bile, centrul liber. O mutare: o bilă sare ortogonal peste o vecină într-o gaură liberă și bila sărită se scoate. Duel: fiecare joacă propria tablă identică, pe rând; cine rămâne fără mutări iese din joc. La final câștigă cine are mai puține bile (egal: ultima bilă în centru). Mutări: d2-d4.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
