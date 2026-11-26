# Five Hundred (500, doi jucători) (`fivehundred`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play fivehundred --seed 1`. Sursa implementării: `polytabletop-games/src/fivehundred.rs`.

> Rezumat din `polytabletop list`: 500 pentru 2 jucători: 43 de cărți (cu joker), 10 în mână, 5 grămezi de câte 2 (jos + sus) și un kitty de 3. Licitezi 6–10 levate cu atu sau fără atu (fără misere); contractantul ia kitty-ul și aruncă 3 cărți. Levate de 4 cărți (câte una din mână și una cu fața în sus); jokerul și valeții (bower) sunt atuurile cele mai mari. Contract reușit = valoarea lui, eșuat = −valoarea; adversarul ia 10 pe levată; 500 / −500. Informație ascunsă. Mutări: licit 7h, licit 8nt, pas, descarta As, 10h, Jk:h, urmatoarea.

## Reguli implementate

500 pentru 2 jucători: 43 de cărți (cu joker), 10 în mână, 5 grămezi de câte 2 (jos + sus) și un kitty de 3. Licitezi 6–10 levate cu atu sau fără atu (fără misere); contractantul ia kitty-ul și aruncă 3 cărți. Levate de 4 cărți (câte una din mână și una cu fața în sus); jokerul și valeții (bower) sunt atuurile cele mai mari. Contract reușit = valoarea lui, eșuat = −valoarea; adversarul ia 10 pe levată; 500 / −500. Informație ascunsă. Mutări: licit 7h, licit 8nt, pas, descarta As, 10h, Jk:h, urmatoarea.

## Variante alese și limite

Doar varianta cu 2 jucători, fără misère și fără open misère; jokerul ca atu cel mai mare, cu culoare declarată când e jucat în fără-atu. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: în modul hotseat ecranul se ascunde între jucători.

## Calculatorul

Adversarul AI caută în arbore (alpha-beta) cu o funcție de evaluare care nu citește informația ascunsă; nu trișează.
