# Nonogram (Picross), duel (`nonogram`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play nonogram --seed 1`. Sursa implementării: `polytabletop-games/src/nonogram.rs`.

> Rezumat din `polytabletop list`: Doi jucători rezolvă aceeași imagine ascunsă 8×8 după indiciile rândurilor și coloanelor: revendici o căsuță necunoscută ca plină (a1) sau goală (a1x); apoi verific o dezvăluie. Corect +1, greșit −2. Mutări: a1, a1x, verific.

## Scop

Aduni mai multe puncte revendicând corect căsuțele.

## Pregătire

Grilă 8×8 cu soluție generată din sămânță (nevăzută); indicii pentru fiecare rând și coloană.

## Desfășurarea turei

Alegi o căsuță necunoscută: `a1` (plină) sau `a1x` (goală); `verific` (mutare separată) arată adevărul; +1 corect, −2 greșit; căsuța devine cunoscută pentru amândoi.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `a1`
* `b3x`
* `verific`

## Simplificări și note

AI-ul deduce din indicii și căsuțele cunoscute (probabilitatea fiecărei căsuțe din toate aranjamentele posibile); nu citește soluția.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
