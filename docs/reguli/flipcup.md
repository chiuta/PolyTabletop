# Flip Cup (`flipcup`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play flipcup --seed 1`. Sursa implementării: `polytabletop-games/src/flipcup.rs`.

> Rezumat din `polytabletop list`: Duel de relee de petrecere, fără băutură: două echipe de câte 5 trebuie să termine fiecare 5 răsturnări de pahar. Alegi repede (55% reușită, costă 2 timp) sau atent (80%, costă 3); la ratare se repetă, timpul se pierde. Joacă echipa cu ceasul mai mic; câștigă echipa care termină cele 5 răsturnări cu cel mai mic timp. Rezultatul se hotărăște la pasul forțat rastoarna. Mutări: repede, atent, rastoarna.

## Reguli implementate

Duel de relee de petrecere, fără băutură: două echipe de câte 5 trebuie să termine fiecare 5 răsturnări de pahar. Alegi repede (55% reușită, costă 2 timp) sau atent (80%, costă 3); la ratare se repetă, timpul se pierde. Joacă echipa cu ceasul mai mic; câștigă echipa care termină cele 5 răsturnări cu cel mai mic timp. Rezultatul se hotărăște la pasul forțat rastoarna. Mutări: repede, atent, rastoarna.

## Variante alese și limite

Joc de petrecere fără băutură: 5 membri pe echipă în relay, șanse fixe de reușită și costuri de timp; rezultatul încercării la pasul forțat `rastoarna`. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
