# Crichet cu zaruri (Howzat!) (`howzat`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play howzat --seed 1`. Sursa implementării: `polytabletop-games/src/howzat.rs`.

> Rezumat din `polytabletop list`: Cricket cu zaruri pentru doi: 2 reprize de 30 de mingi și 5 wicket-uri; J1 bate primul, J2 urmărește. La fiecare minge bătătorul alege «lovesc» (atac) sau «apar» (apărare), apoi aruncătorul dă «arunc»: atac 1–2 out, 3=1, 4=2, 5=4, 6=6 runs; apărare 1 out, 2–3=0, 4–5=1, 6=2 runs. Câștigă cine are mai multe runs. Mutări: lovesc, apar, arunc.

## Reguli implementate

Cricket cu zaruri pentru doi: 2 reprize de 30 de mingi și 5 wicket-uri; J1 bate primul, J2 urmărește. La fiecare minge bătătorul alege «lovesc» (atac) sau «apar» (apărare), apoi aruncătorul dă «arunc»: atac 1–2 out, 3=1, 4=2, 5=4, 6=6 runs; apărare 1 out, 2–3=0, 4–5=1, 6=2 runs. Câștigă cine are mai multe runs. Mutări: lovesc, apar, arunc.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
