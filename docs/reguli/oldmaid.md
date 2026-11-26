# Old Maid (Bătrâna fată) (`oldmaid`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play oldmaid --seed 1`. Sursa implementării: `polytabletop-games/src/oldmaid.rs`.

> Rezumat din `polytabletop list`: Bătrâna fată pentru doi: pachet de 52 de cărți din care lipsește o damă, deci o damă rămâne fără pereche. Se aruncă perechile de aceeași valoare; apoi tragi pe nevăzute din mâna celuilalt (trag N = a N-a carte) și arunci perechile. Când rămâne o carte, cine o ține (dama) pierde. Mâini ascunse; rezultatul apare la pasul final. Mutări: trag N, final.

## Reguli implementate

Bătrâna fată pentru doi: pachet de 52 de cărți din care lipsește o damă, deci o damă rămâne fără pereche. Se aruncă perechile de aceeași valoare; apoi tragi pe nevăzute din mâna celuilalt (trag N = a N-a carte) și arunci perechile. Când rămâne o carte, cine o ține (dama) pierde. Mâini ascunse; rezultatul apare la pasul final. Mutări: trag N, final.

## Variante alese și limite

Mâna celui din care se trage se amestecă la loc, ca poziția să nu poată fi memorată; AI-ul alege poziția la întâmplare. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: da (HOTSEAT_SAFE). La hot-seat se afișează doar informația jucătorului la mutare; AI-ul citește doar mâna lui și informațiile publice, iar tragerile/zarurile/dezvăluirile sunt pași forțați (nu poate trișa).

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
