# Renju (`renju`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play renju --seed 1`. Sursa implementării: `polytabletop-games/src/renju.rs`.

> Rezumat din `polytabletop list`: Gomoku pe 15×15 cu restricții pentru Negru (J1): linia lungă (6+), dublu-patru și dublu-trei deschis sunt interzise; Negru câștigă cu exact cinci, Alb cu cinci sau mai multe. Mutări: h8 (coloane a–o, rânduri 1–15).

## Scop

Aliniezi cinci piese (Negru: exact cinci; Alb: cinci sau mai multe).

## Pregătire

Tablă 15×15 goală; Negru (J1) începe, prima mutare e în centru (h8).

## Desfășurarea turei

Pui o piesă pe o intersecție liberă. Negrul nu poate juca punctele interzise (6+, 4-4, 3-3 deschis); acestea nu apar în lista mutărilor.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `h8`
* `h9`

## Simplificări și note

Fără regulile de deschidere (swap etc.); la „trei deschis” nu se verifică recursiv dacă patru-ul ar fi interzis. AI: alfa-beta cu limită de timp (rezultatele pot varia între rulări).

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
