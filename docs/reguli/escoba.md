# Escoba (Escoba de 15) (`escoba`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play escoba --seed 1`. Sursa implementării: `polytabletop-games/src/escoba.rs`.

> Rezumat din `polytabletop list`: Baraja spaniolă de 40 de cărți (valori 1–7, 10=8, 11=9, 12=10), 3 în mână și 4 pe masă. Joci o carte și poți lua cărți de pe masă care, adunate cu ea, dau 15; golești masa = escoba (1 punct, nu la ultima carte). Punctaj la capăt de mână: cartas, oros, siete de velo (7o), setenta și escobe; primul la 21. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: 7o, 7o:4c+4e, imparte, urmatoarea.

## Reguli implementate

Baraja spaniolă de 40 de cărți (valori 1–7, 10=8, 11=9, 12=10), 3 în mână și 4 pe masă. Joci o carte și poți lua cărți de pe masă care, adunate cu ea, dau 15; golești masa = escoba (1 punct, nu la ultima carte). Punctaj la capăt de mână: cartas, oros, siete de velo (7o), setenta și escobe; primul la 21. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: 7o, 7o:4c+4e, imparte, urmatoarea.

## Variante alese și limite

Capturarea este opțională (poți pune cartea jos chiar dacă ai putea lua); fără escoba la ultima carte. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: în modul hotseat ecranul se ascunde între jucători.

## Calculatorul

Adversarul AI caută în arbore (alpha-beta) cu o funcție de evaluare care nu citește informația ascunsă; nu trișează.
