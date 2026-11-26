# Cờ gánh (`coganh`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play coganh --seed 1`. Sursa implementării: `polytabletop-games/src/coganh.rs`.

> Rezumat din `polytabletop list`: Joc vietnamez de capturi pe tabla 5×5 cu diagonale; 8 piese fiecare. Muți un pas pe o linie către un punct liber. Capturile schimbă culoarea pieselor: gánh (piesa mutată încadrează o piesă adversă pe o linie) și vây (un grup advers fără niciun punct liber în jur). Câștigi lăsând adversarul fără piese sau mutări. Mutări: a1b2 (de la, la).

## Reguli implementate

Joc vietnamez de capturi pe tabla 5×5 cu diagonale; 8 piese fiecare. Muți un pas pe o linie către un punct liber. Capturile schimbă culoarea pieselor: gánh (piesa mutată încadrează o piesă adversă pe o linie) și vây (un grup advers fără niciun punct liber în jur). Câștigi lăsând adversarul fără piese sau mutări. Mutări: a1b2 (de la, la).

## Variante alese și limite

Fără chém (capturarea obligatorie prin golul lăsat). Capturile schimbă culoarea pieselor (gánh, vây). Remiză după 100 de mutări fără captură cu număr egal de piese. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
