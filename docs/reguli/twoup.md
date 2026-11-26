# Two-up (Australia) (`twoup`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play twoup --seed 1`. Sursa implementării: `polytabletop-games/src/twoup.rs`.

> Rezumat din `polytabletop list`: Spinnerul mizează pe stemă (5, 10 sau 20 de jetoane); celălalt acoperă. Două monede: cap-cap = spinnerul câștigă și continuă; pajura-pajura = pierde și rolul trece; cap-pajura (odds) = se aruncă din nou, cinci odds la rând = spinnerul pierde. 24 de mize, start 50 de jetoane; câștigă cine are mai multe. Mutări: miza 5, miza 10, miza 20, arunc.

## Reguli implementate

Spinnerul mizează pe stemă (5, 10 sau 20 de jetoane); celălalt acoperă. Două monede: cap-cap = spinnerul câștigă și continuă; pajura-pajura = pierde și rolul trece; cap-pajura (odds) = se aruncă din nou, cinci odds la rând = spinnerul pierde. 24 de mize, start 50 de jetoane; câștigă cine are mai multe. Mutări: miza 5, miza 10, miza 20, arunc.

## Variante alese și limite

Format simplificat: 24 de mize, 50 de jetoane; cinci «odds» la rând fac spinnerul să piardă. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație completă (fără cărți ascunse).

## Calculatorul

Adversarul AI caută în arbore (alpha-beta) cu o funcție de evaluare care nu citește informația ascunsă; nu trișează.
