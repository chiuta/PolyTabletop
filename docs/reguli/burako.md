# Burako (Buraco) (`burako`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play burako --seed 1`. Sursa implementării: `polytabletop-games/src/burako.rs`.

> Rezumat din `polytabletop list`: 106 cărți (2 pachete + 2 jokeri), 11 în mână și un muerto de 11. Tragi din stoc (mazo) sau iei tot pozo, lași jos scări (≥3 consecutive, aceeași culoare) și piernas (≥3 de același rang), cel mult un comodin (joker sau 2) per combinație, apoi arunci. Canastă = 7+ cărți: pură 200, impură 100; închizi cu muertoul luat și o canastă (+100). Meci până la 1500 sau 4 mâini. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: mazo, pozo, esc:4h+5h+6h, set:7h+7c+Jk, +1:8h, descarta 5h, muerto, urmatoarea.

## Reguli implementate

106 cărți (2 pachete + 2 jokeri), 11 în mână și un muerto de 11. Tragi din stoc (mazo) sau iei tot pozo, lași jos scări (≥3 consecutive, aceeași culoare) și piernas (≥3 de același rang), cel mult un comodin (joker sau 2) per combinație, apoi arunci. Canastă = 7+ cărți: pură 200, impură 100; închizi cu muertoul luat și o canastă (+100). Meci până la 1500 sau 4 mâini. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: mazo, pozo, esc:4h+5h+6h, set:7h+7c+Jk, +1:8h, descarta 5h, muerto, urmatoarea.

## Variante alese și limite

Pachet de 106 cărți, meci la 1500 sau 4 mâini, un singur comodin per combinație, fără schimbul comodinului; variantă simplificată a regulamentului argentinian, doi jucători. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: în modul hotseat ecranul se ascunde între jucători.

## Calculatorul

Adversarul AI caută în arbore (alpha-beta) cu o funcție de evaluare care nu citește informația ascunsă; nu trișează.
