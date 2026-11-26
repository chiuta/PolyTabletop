# Trictrac (simplificat) (`trictrac`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play trictrac --seed 1`. Sursa implementării: `polytabletop-games/src/trictrac.rs`.

> Rezumat din `polytabletop list`: Trictrac simplificat: 24 de puncte, câte 15 piese stivuite pe punctul de start, două zaruri, fără lovire. Scoți piese doar când toate sunt în ultimul sfert. Punctaj în găuri la sfârșitul turei: dublă +2, trei puncte din ultimul sfert cu 2+ piese +4, fiecare piesă scoasă +1. Primul la 15 găuri câștigă. Sistemul real de găuri (jan-uri, battre à vrai) nu e implementat. Mutări: arunc, zar:punct (ex. 4:1).

## Reguli implementate

Trictrac simplificat: 24 de puncte, câte 15 piese stivuite pe punctul de start, două zaruri, fără lovire. Scoți piese doar când toate sunt în ultimul sfert. Punctaj în găuri la sfârșitul turei: dublă +2, trei puncte din ultimul sfert cu 2+ piese +4, fiecare piesă scoasă +1. Primul la 15 găuri câștigă. Sistemul real de găuri (jan-uri, battre à vrai) nu e implementat. Mutări: arunc, zar:punct (ex. 4:1).

## Variante alese și limite

Variantă simplificată cu punctaj redus: scheletul zarurilor și al punctelor, fără sistemul complet de jan-uri, „battre à vrai/à faux” și plăți.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
