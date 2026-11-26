# Bo Bing (Pua Tiong Chiu) (`bobing`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play bobing --seed 1`. Sursa implementării: `polytabletop-games/src/bobing.rs`.

> Rezumat din `polytabletop list`: Joc cu zaruri de Festivalul Lunii din Xiamen: șase zaruri într-un castron, premii (63 de prăjituri de lună) după combinație: Zhuangyuan cea mai bună, apoi Duitang, Sanhong, Sijin, Erju, Yixiu (după numărul de 4). Fiecare categorie are rezervă limitată; câștigă cine are mai multe puncte după 12 aruncări fiecare. Mutare: arunc.

## Scop

Aduni mai multe puncte de clasament după 12 aruncări pe jucător.

## Pregătire

63 de premii (prăjituri de lună) în rezerve pe categorii; puncte de clasament 32/16/8/4/2/1 pentru Zhuangyuan … Yixiu.

## Desfășurarea turei

Singura mutare e `arunc`: 6 zaruri într-un castron; combinația nimerită ia un premiu din rezerva ei (dacă rezerva e goală, nu câștigi). Se termină după 12 aruncări pe jucător sau când se golesc rezervele.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `arunc`

## Simplificări și note

Variantă simplificată: fără „urmărire” (追饼) și fără premii mari pe sub-combinații. Nu există decizii: jocul este de noroc.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
