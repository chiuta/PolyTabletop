# Table skittles (popice de masă) (`skittles`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play skittles --seed 1`. Sursa implementării: `polytabletop-games/src/skittles.rs`.

> Rezumat din `polytabletop list`: Popice de masă cu norocul în locul îndemânării: 9 popice într-o grilă 3×3. La fiecare aruncare țintești o coloană (stanga/mijloc/dreapta), apoi «lansez»; popicele din coloana țintă cad cu 60 %, vecine 25 %, opuse 8 %. O tură are 3 aruncări; fiecare popică 1 punct (cea din mijloc 2); toate nouă = 5 bonus. 6 ture fiecare; câștigă cine are mai multe puncte.

## Reguli implementate

Popice de masă cu norocul în locul îndemânării: 9 popice într-o grilă 3×3. La fiecare aruncare țintești o coloană (stanga/mijloc/dreapta), apoi «lansez»; popicele din coloana țintă cad cu 60 %, vecine 25 %, opuse 8 %. O tură are 3 aruncări; fiecare popică 1 punct (cea din mijloc 2); toate nouă = 5 bonus. 6 ture fiecare; câștigă cine are mai multe puncte.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
