# Șotron (Hopscotch) (`sotron`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sotron --seed 1`. Sursa implementării: `polytabletop-games/src/sotron.rs`.

> Rezumat din `polytabletop list`: Joc de copii pentru doi: cursă pe pătratele 1–10. La tura ta arunci piatra: prudent (țintești pătratul următor: 80% reușită, +1) sau indraznet (peste un pătrat: 50%, +2; la ratare cazi un pătrat înapoi). Câștigă primul care duce piatra la 10 și se întoarce. Aruncarea se hotărăște la pasul forțat sar. Mutări: prudent, indraznet, sar.

## Reguli implementate

Joc de copii pentru doi: cursă pe pătratele 1–10. La tura ta arunci piatra: prudent (țintești pătratul următor: 80% reușită, +1) sau indraznet (peste un pătrat: 50%, +2; la ratare cazi un pătrat înapoi). Câștigă primul care duce piatra la 10 și se întoarce. Aruncarea se hotărăște la pasul forțat sar. Mutări: prudent, indraznet, sar.

## Variante alese și limite

Cursă simplificată pe pătratele 1–10 cu șanse fixe; aruncarea se hotărăște la pasul forțat `sar`. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
