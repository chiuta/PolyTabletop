# Craps (`craps`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play craps --seed 1`. Sursa implementării: `polytabletop-games/src/craps.rs`.

> Rezumat din `polytabletop list`: Duel la masa de zaruri pentru doi: shooterul alege miza (5, 10, 20) și pariul, pass sau dont (don't pass); celălalt acoperă pariul opus. Come-out: 7 sau 11 câștigă pass; 2, 3, 12 = craps (don't pass câștigă la 2 și 3, la 12 miza se anulează); orice alt total devine punct, se aruncă până apare punctul (pass) sau 7 (don't pass). 16 runde, start 100 de jetoane, câștigă cine are mai multe. Mutări: pass 10, dont 5, arunc.

## Reguli implementate

Duel la masa de zaruri pentru doi: shooterul alege miza (5, 10, 20) și pariul, pass sau dont (don't pass); celălalt acoperă pariul opus. Come-out: 7 sau 11 câștigă pass; 2, 3, 12 = craps (don't pass câștigă la 2 și 3, la 12 miza se anulează); orice alt total devine punct, se aruncă până apare punctul (pass) sau 7 (don't pass). 16 runde, start 100 de jetoane, câștigă cine are mai multe. Mutări: pass 10, dont 5, arunc.

## Variante alese și limite

Duel între doi jucători (fără bancă): pass/don't pass, miza 5/10/20, 16 runde; fără alte pariuri (come, odds, field). Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
