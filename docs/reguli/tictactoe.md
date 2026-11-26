# X și 0 (Tic-tac-toe) (`tictactoe`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tictactoe --seed 1`. Sursa implementării: `polytabletop-games/src/tictactoe.rs`.

> Rezumat din `polytabletop list`: Tablă 3×3; jucătorii pun pe rând X și 0. Trei la rând (linie, coloană sau diagonală) câștigă; tabla plină fără linie e remiză. Mutări: a1…c3.

## Reguli implementate

Tablă 3×3; jucătorii pun pe rând X și 0. Trei la rând (linie, coloană sau diagonală) câștigă; tabla plină fără linie e remiză. Mutări: a1…c3.

## Variante alese și limite

Joc rezolvat: AI-ul explorează tot arborele, remiza e rezultatul cu joc corect. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
