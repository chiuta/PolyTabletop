# Petteia (`petteia`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play petteia --seed 1`. Sursa implementării: `polytabletop-games/src/petteia.rs`.

> Rezumat din `polytabletop list`: Reconstituire a jocului grecesc antic pe 8×8: 16 piese fiecare, orice piesă merge ca un turn; o piesă e capturată când mutarea ta o prinde între două ale tale pe un rând sau o coloană (mai multe capturi într-o mutare). Pierzi cu o singură piesă sau fără mutări; după 60 de mutări fără captură câștigă cine are mai multe piese. Mutări: a2a4 (de la, la).

## Reguli implementate

Reconstituire a jocului grecesc antic pe 8×8: 16 piese fiecare, orice piesă merge ca un turn; o piesă e capturată când mutarea ta o prinde între două ale tale pe un rând sau o coloană (mai multe capturi într-o mutare). Pierzi cu o singură piesă sau fără mutări; după 60 de mutări fără captură câștigă cine are mai multe piese. Mutări: a2a4 (de la, la).

## Variante alese și limite

Reconstituire (regulile antice nu s-au păstrat): captură custodială, piesă ca turnul, 60 de mutări fără captură = câștigă cine are mai multe piese. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
