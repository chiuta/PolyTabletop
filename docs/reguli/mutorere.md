# Mu Torere (Noua Zeelandă) (`mutorere`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mutorere --seed 1`. Sursa implementării: `polytabletop-games/src/mutorere.rs`.

> Rezumat din `polytabletop list`: 8 puncte pe cerc (kewai 1–8) și centrul (0); fiecare are 4 piese pe puncte vecine, centrul gol. Muți o piesă pe un punct vecin gol; în centru intri doar cu o piesă vecină cu o piesă adversă. Pierde cine nu mai poate muta; remiză la a treia repetare sau după 200 de mutări. Mutări: 3-4 (de pe 3 pe 4), 4-0 (în centru), 0-6 (din centru).

## Reguli implementate

8 puncte pe cerc (kewai 1–8) și centrul (0); fiecare are 4 piese pe puncte vecine, centrul gol. Muți o piesă pe un punct vecin gol; în centru intri doar cu o piesă vecină cu o piesă adversă. Pierde cine nu mai poate muta; remiză la a treia repetare sau după 200 de mutări. Mutări: 3-4 (de pe 3 pe 4), 4-0 (în centru), 0-6 (din centru).

## Variante alese și limite

Remiză la a treia repetare a poziției sau după 200 de mutări. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație completă (fără cărți ascunse).

## Calculatorul

Adversarul AI caută în arbore (alpha-beta) cu o funcție de evaluare care nu citește informația ascunsă; nu trișează.
