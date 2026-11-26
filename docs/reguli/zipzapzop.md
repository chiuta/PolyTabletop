# Zip Zap Zop (`zipzapzop`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play zipzapzop --seed 1`. Sursa implementării: `polytabletop-games/src/zipzapzop.rs`.

> Rezumat din `polytabletop list`: Joc de numărat: numărați pe rând de la 1 la 30; spuneți zip la multiplii de 3, zap la multiplii de 5, zop la multiplii de 15. Un cuvânt greșit pierde. Fiecare are 2 pas-uri (numărul rămâne adversarului). Câștigă cine spune corect 30. Mutări: numărul, zip, zap, zop, pas.

## Reguli implementate

Joc de numărat: numărați pe rând de la 1 la 30; spuneți zip la multiplii de 3, zap la multiplii de 5, zop la multiplii de 15. Un cuvânt greșit pierde. Fiecare are 2 pas-uri (numărul rămâne adversarului). Câștigă cine spune corect 30. Mutări: numărul, zip, zap, zop, pas.

## Variante alese și limite

Pas-urile (2 per jucător) adaugă strategie: fără ele jocul e decis de paritate.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
