# Hazard (strămoșul craps-ului) (`hazard`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play hazard --seed 1`. Sursa implementării: `polytabletop-games/src/hazard.rs`.

> Rezumat din `polytabletop list`: Aruncătorul strigă un main 5–9 și o miză. Prima aruncare cu două zaruri: main = nick (câștigă); 2 sau 3 = pierde; 11 și 12 depind de main (5/9: ambele pierd; 6/8: 11 pierde, 12 nick; 7: 11 nick, 12 pierde); orice altceva devine chance. Apoi se aruncă până iese chance (câștigă) sau main (pierde). Zarurile trec după trei pierderi la rând. 100 de jetoane, 20 de mâini. Mutări: main 7 10, arunc.

## Reguli implementate

Aruncătorul strigă un main 5–9 și o miză. Prima aruncare cu două zaruri: main = nick (câștigă); 2 sau 3 = pierde; 11 și 12 depind de main (5/9: ambele pierd; 6/8: 11 pierde, 12 nick; 7: 11 nick, 12 pierde); orice altceva devine chance. Apoi se aruncă până iese chance (câștigă) sau main (pierde). Zarurile trec după trei pierderi la rând. 100 de jetoane, 20 de mâini. Mutări: main 7 10, arunc.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
