# Dice Baseball (`dicebaseball`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play dicebaseball --seed 1`. Sursa implementării: `polytabletop-games/src/dicebaseball.rs`.

> Rezumat din `polytabletop list`: Baseball cu două zaruri pentru doi, fără decizii: 9 reprize, J1 bate apoi J2 până la trei outuri; reprize suplimentare (cel mult 12) la egalitate. Totalul fiecărei aruncări e un rezultat: 2 aruncare ratată, 3 triplu, 4 dublu, 5 walk, 6 fly out, 7 ground out, 8 simplu, 9 strike out, 10 minge afară (se reia), 11 dublă eliminare, 12 home run. Alergătorii avansează la lovituri; trecerea de baza a treia înscrie. Mutare: arunc.

## Reguli implementate

Baseball cu două zaruri pentru doi, fără decizii: 9 reprize, J1 bate apoi J2 până la trei outuri; reprize suplimentare (cel mult 12) la egalitate. Totalul fiecărei aruncări e un rezultat: 2 aruncare ratată, 3 triplu, 4 dublu, 5 walk, 6 fly out, 7 ground out, 8 simplu, 9 strike out, 10 minge afară (se reia), 11 dublă eliminare, 12 home run. Alergătorii avansează la lovituri; trecerea de baza a treia înscrie. Mutare: arunc.

## Variante alese și limite

Fără decizii: tabelul de rezultate pe totalul zarurilor e fixat de implementare; max. 12 reprize. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
