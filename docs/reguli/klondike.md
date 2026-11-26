# Solitaire (Klondike), duel (`klondike`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play klondike --seed 1`. Sursa implementării: `polytabletop-games/src/klondike.rs`.

> Rezumat din `polytabletop list`: Klondike clasic (tragere de câte o carte), ca duel: fiecare are propria amestecare și face câte o mutare; câștigă primul care duce toate cele 52 de cărți pe fundații. Mutări: d, w>f, w>3, 3>f, 3>5:2.

## Scop

Duci primul toate cele 52 de cărți pe fundații (sau ai mai multe după 240 de mutări).

## Pregătire

Fiecare jucător are masa lui: 7 coloane (1…7 cărți, ultima cu fața în sus), 4 fundații, pachet și deșeu.

## Desfășurarea turei

O mutare pe tură pe masa ta: `d` (tragi din pachet / întorci deșeul), `w>f` (deșeu → fundație), `w>3` (deșeu → coloana 3), `3>f`, `3>5:2` (două cărți din coloana 3 pe 5). Coloane descrescător, culori alternate; în loc gol doar rege.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `d`
* `w>f`
* `3>5:2`

## Simplificări și note

Cele două mese au amestecări diferite, ca să nu se afle cărți ascunse. AI-ul nu cunoaște cărțile cu fața în jos.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
