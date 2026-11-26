# Fia-med-knuff (`fia`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play fia --seed 1`. Sursa implementării: `polytabletop-games/src/fia.rs`.

> Rezumat din `polytabletop list`: Ludo suedez: 4 piese fiecare, pistă de 48 de câmpuri. 1 sau 6 intră o piesă; 6 dă încă o aruncare; aterizând pe o piesă adversă o trimiți acasă. Finalul cere aruncare exactă; câștigă primul cu toate cele patru acasă. Mutări: intra sau numărul progresului piesei (ex. 12).

## Reguli implementate

Ludo suedez: 4 piese fiecare, pistă de 48 de câmpuri. 1 sau 6 intră o piesă; 6 dă încă o aruncare; aterizând pe o piesă adversă o trimiți acasă. Finalul cere aruncare exactă; câștigă primul cu toate cele patru acasă. Mutări: intra sau numărul progresului piesei (ex. 12).

## Variante alese și limite

Motor comun `ludofam` (reguli Fia): doi jucători pe pătrate opuse, fără obligația de a lovi. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
