# Conkers (`conkers`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play conkers --seed 1`. Sursa implementării: `polytabletop-games/src/conkers.rs`.

> Rezumat din `polytabletop list`: Joc britanic de curte pentru doi: fiecare castană are 6 puncte de rezistență. La tura ta alegi usor (70% reușită, 1 pagubă) sau tare (45% reușită, 2 pagube; la ratare sfoara se poate încurca și pierzi 1). Prima castană ajunsă la 0 se sparge și pierde; după 60 de lovituri câștigă castana mai întreagă. Rezultatul se hotărăște la pasul forțat lovitura. Mutări: usor, tare, lovitura.

## Reguli implementate

Joc britanic de curte pentru doi: fiecare castană are 6 puncte de rezistență. La tura ta alegi usor (70% reușită, 1 pagubă) sau tare (45% reușită, 2 pagube; la ratare sfoara se poate încurca și pierzi 1). Prima castană ajunsă la 0 se sparge și pierde; după 60 de lovituri câștigă castana mai întreagă. Rezultatul se hotărăște la pasul forțat lovitura. Mutări: usor, tare, lovitura.

## Variante alese și limite

Șanse fixe (usor 70%, tare 45%); lovitura se hotărăște la pasul forțat `lovitura`. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
