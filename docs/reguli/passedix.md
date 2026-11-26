# Passe-dix (`passedix`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play passedix --seed 1`. Sursa implementării: `polytabletop-games/src/passedix.rs`.

> Rezumat din `polytabletop list`: Joc francez cu trei zaruri pentru doi: banca (alternativ) alege miza (5, 10, 20) și partea, passe (total 11–18) sau manque (3–10); celălalt acoperă partea opusă; tripletele se re-aruncă. 20 de runde, start 100 de jetoane, câștigă cine are mai multe. Aruncarea e pasul forțat arunc. Mutări: passe 10, manque 5, arunc.

## Reguli implementate

Joc francez cu trei zaruri pentru doi: banca (alternativ) alege miza (5, 10, 20) și partea, passe (total 11–18) sau manque (3–10); celălalt acoperă partea opusă; tripletele se re-aruncă. 20 de runde, start 100 de jetoane, câștigă cine are mai multe. Aruncarea e pasul forțat arunc. Mutări: passe 10, manque 5, arunc.

## Variante alese și limite

Doi jucători (banca alternează), 20 de runde, miza 5/10/20; tripletele se re-aruncă. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
