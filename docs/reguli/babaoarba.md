# Baba-oarba (`babaoarba`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play babaoarba --seed 1`. Sursa implementării: `polytabletop-games/src/babaoarba.rs`.

> Rezumat din `polytabletop list`: Duel asimetric pe tablă 7×7: J1 e baba cu ochii legați, J2 fugarul. Baba mută (n, s, e, v, ne…, sta) sau încearcă să prindă o căsuță vecină (prind c4, apoi arat). După fiecare rundă aude un indiciu: cald (fugarul e la cel mult 2 căsuțe) sau rece. Fugarul face un pas. Baba câștigă prinzând fugarul; fugarul câștigă rezistând 30 de runde. Poziția fugarului e ascunsă.

## Reguli implementate

Duel asimetric pe tablă 7×7: J1 e baba cu ochii legați, J2 fugarul. Baba mută (n, s, e, v, ne…, sta) sau încearcă să prindă o căsuță vecină (prind c4, apoi arat). După fiecare rundă aude un indiciu: cald (fugarul e la cel mult 2 căsuțe) sau rece. Fugarul face un pas. Baba câștigă prinzând fugarul; fugarul câștigă rezistând 30 de runde. Poziția fugarului e ascunsă.

## Variante alese și limite

Joc de grup redus la un duel baba/fugar; baba își ține o hartă de posibilități actualizată doar din indicii (nu citește poziția adevărată).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: `set_viewer` este respectat (mod hot-seat sigur).

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
