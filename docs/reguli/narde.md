# Narde scurtă (Short Nardy) (`narde`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play narde --seed 1`. Sursa implementării: `polytabletop-games/src/narde.rs`.

> Rezumat din `polytabletop list`: Narde scurtă (rudă cu table): câte 15 piese, două zaruri (la dublă se joacă de patru ori), aduci toate piesele acasă și le scoți primul. Implementarea folosește motorul de table cu regulile standard. Mutări: arunc, apoi mutările pe puncte.

## Reguli implementate

Narde scurtă (rudă cu table): câte 15 piese, două zaruri (la dublă se joacă de patru ori), aduci toate piesele acasă și le scoți primul. Implementarea folosește motorul de table cu regulile standard. Mutări: arunc, apoi mutările pe puncte.

## Variante alese și limite

Încărcat pe motorul de table (backgammon): nu reproduce exact start-ul și direcția narde-lor rusești/persane; e o variantă înrudită.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
