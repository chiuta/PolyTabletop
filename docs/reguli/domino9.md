# Dominó cubano (doble nueve) (`domino9`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play domino9 --seed 1`. Sursa implementării: `polytabletop-games/src/domino9.rs`.

> Rezumat din `polytabletop list`: Set dublu-nouă de 55 de piese; câte 10, 35 în rezervă. Pui o piesă care se potrivește la un capăt (începe cel cu cel mai mare dublu); dacă nu ai piesă, tragi până poți, iar cu rezerva goală treci. Câștigă cine rămâne primul fără piese; la blocaj, cine are mai puține puncte. Mutări: piesa cu capătul (ex. 5-7 stanga), trage, pas. Mâini și rezervă ascunse.

## Reguli implementate

Set dublu-nouă de 55 de piese; câte 10, 35 în rezervă. Pui o piesă care se potrivește la un capăt (începe cel cu cel mai mare dublu); dacă nu ai piesă, tragi până poți, iar cu rezerva goală treci. Câștigă cine rămâne primul fără piese; la blocaj, cine are mai puține puncte. Mutări: piesa cu capătul (ex. 5-7 stanga), trage, pas. Mâini și rezervă ascunse.

## Variante alese și limite

Varianta cubaneză cu rezervă (35 de piese); primul jucător deschide cu cel mai mare dublu.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: `set_viewer` este respectat (mod hot-seat sigur).

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
