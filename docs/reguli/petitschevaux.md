# Jeu des petits chevaux (`petitschevaux`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play petitschevaux --seed 1`. Sursa implementării: `polytabletop-games/src/petitschevaux.rs`.

> Rezumat din `polytabletop list`: Ludo francez cu cai: 4 cai fiecare, pistă de 56 de câmpuri și 6 câmpuri de final. Un 6 scoate un cal din grajd și dă încă o aruncare; câmpul tău de start e sigur; aterizând pe un cal adversar îl trimiți înapoi; sosirea cere aruncare exactă. Mutări: intra sau numărul progresului calului (ex. 12).

## Reguli implementate

Ludo francez cu cai: 4 cai fiecare, pistă de 56 de câmpuri și 6 câmpuri de final. Un 6 scoate un cal din grajd și dă încă o aruncare; câmpul tău de start e sigur; aterizând pe un cal adversar îl trimiți înapoi; sosirea cere aruncare exactă. Mutări: intra sau numărul progresului calului (ex. 12).

## Variante alese și limite

Motor comun `ludofam`: doi jucători, fără pătrate-stea și fără bariere din două piese. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
