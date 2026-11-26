# Hus (Namibia) (`hus`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play hus --seed 1`. Sursa implementării: `polytabletop-games/src/hus.rs (+ mancala4.rs)`.

> Rezumat din `polytabletop list`: Mancala 4 × 8, 2 semințe în fiecare groapă. Muți dintr-o groapă proprie cu ≥ 2 semințe; ultima sămânță într-o groapă ocupată: din rândul din față cu groapa opusă ocupată captezi coloana adversă (față + spate) și o semeni mai departe; altfel continui lanțul. Pierde cine nu mai are nicio groapă cu ≥ 2. Mutări: 3 (față), b3 (spate).

## Scop

Adversarul rămâne fără nicio groapă cu cel puțin 2 semințe.

## Pregătire

Tablă 4 × 8, două rânduri unite într-un inel de 16 gropi pe jucător, 2 semințe în fiecare groapă (64 în total).

## Desfășurarea turei

Muți dintr-o groapă proprie cu cel puțin 2 semințe, câte una în fiecare groapă următoare (sens fix). Dacă ultima sămânță cade într-o groapă deja ocupată: din rândul din față cu groapa opusă ocupată, capturezi ambele gropi adverse din acea coloană (față + spate) și le semeni mai departe; altfel ridici groapa și continui (lanț). Într-o groapă goală tura se termină.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `3`: groapa 3 din rândul din față; `b3`: groapa 3 din rândul din spate (coloane din punctul tău de vedere).

## Reguli speciale și variante alese

* Captura ia coloana adversă întreagă (față + spate).

## Punctaj

Nu există punctaj.

## Sfârșitul jocului

Pierde cine nu mai are nicio groapă cu 2+ semințe la mutare. După 800 de mutări câștigă cine are mai multe semințe.

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă 5.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar nu are informație ascunsă.

## Diferențe față de regulile tradiționale

* Locul de unde se seamănă semințele capturate (aici: de la groapa următoare, în același sens) diferă între surse; este varianta aleasă.
* Lanțul e limitat la 3000 de semințe.
