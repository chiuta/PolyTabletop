# Tangram (duel pe grilă) (`tangram`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tangram --seed 1`. Sursa implementării: `polytabletop-games/src/tangram.rs`.

> Rezumat din `polytabletop list`: Duel Tangram simplificat pe grilă 6×6: fiecare jucător are 7 piese (2 triunghiuri mici = 1 pătrățel, triunghi mediu = domino, pătrat = 2×2, paralelogram = S de 4, 2 triunghiuri mari = L de 3). Puneți alternativ câte o piesă (orice rotire sau oglindire), fără suprapunere; cine nu mai poate pune nicio piesă pierde. Mutări: piesă+orientare@colț (ex. q1@c3). Nu e puzzle-ul cu siluete.

## Reguli implementate

Duel Tangram simplificat pe grilă 6×6: fiecare jucător are 7 piese (2 triunghiuri mici = 1 pătrățel, triunghi mediu = domino, pătrat = 2×2, paralelogram = S de 4, 2 triunghiuri mari = L de 3). Puneți alternativ câte o piesă (orice rotire sau oglindire), fără suprapunere; cine nu mai poate pune nicio piesă pierde. Mutări: piesă+orientare@colț (ex. q1@c3). Nu e puzzle-ul cu siluete.

## Variante alese și limite

Variantă pe grilă: piesele sunt poliomino cu arii apropiate; nu sunt siluete și unghiuri de 45°.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
