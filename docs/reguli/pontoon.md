# Pontoon (`pontoon`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pontoon --seed 1`. Sursa implementării: `polytabletop-games/src/pontoon.rs`.

> Rezumat din `polytabletop list`: Banca (rolul alternează la fiecare mână) contra jucătorului: Pontoon (as + carte de 10) > Five Card Trick (5 cărți ≤ 21) > 21 > cât mai aproape de 21; egalitatea o câștigă banca. Jucătorul: carte (twist), cumpar (carte ascunsă, miza +10, doar înainte de carte), stai (de la 15); banca stă de la 17. Pontoon și Five Card Trick plătesc dublu. 20 de mâini cu miza 10, câștigă cine are mai multe jetoane. Informație ascunsă. Mutări: carte, cumpar, stai, decontare.

## Reguli implementate

Banca (rolul alternează la fiecare mână) contra jucătorului: Pontoon (as + carte de 10) > Five Card Trick (5 cărți ≤ 21) > 21 > cât mai aproape de 21; egalitatea o câștigă banca. Jucătorul: carte (twist), cumpar (carte ascunsă, miza +10, doar înainte de carte), stai (de la 15); banca stă de la 17. Pontoon și Five Card Trick plătesc dublu. 20 de mâini cu miza 10, câștigă cine are mai multe jetoane. Informație ascunsă. Mutări: carte, cumpar, stai, decontare.

## Variante alese și limite

Fără împărțirea perechilor (split); cost fix la cumpărare; banca alternează la fiecare mână. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: în modul hotseat ecranul se ascunde între jucători.

## Calculatorul

Adversarul AI caută în arbore (alpha-beta) cu o funcție de evaluare care nu citește informația ascunsă; nu trișează.
