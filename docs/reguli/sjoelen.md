# Sjoelen (`sjoelen`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sjoelen --seed 1`. Sursa implementării: `polytabletop-games/src/sjoelen.rs`.

> Rezumat din `polytabletop list`: Șuflecatul olandez: 30 de discuri, 3 runde, le împingi în patru porți numerotate (2, 3, 4, 1); doar discurile ratate se aruncă din nou. Fiecare serie completă de patru porți valorează 20 de puncte. Mutări: stanga, centru, dreapta, atent, arunc.

## Scop

După cele 3 runde ai mai multe puncte decât adversarul.

## Pregătire

30 de discuri, patru porți (2, 3, 4, 1); J1 joacă cele 3 runde, apoi J2.

## Desfășurarea turei

Alegi ținta (`stanga`, `centru`, `dreapta`, `atent`), apoi `arunc` (mutare separată). Seriile complete (câte un disc în fiecare poartă) valorează 20; restul după poartă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `centru`
* `arunc`

## Simplificări și note

Probabilitățile fiecărei ținte sunt alegerea noastră. Evaluarea AI nu citește aruncarea următoare.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
