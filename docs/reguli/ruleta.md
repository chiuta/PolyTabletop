# Ruletă (Roulette) (`ruleta`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ruleta --seed 1`. Sursa implementării: `polytabletop-games/src/ruleta.rs`.

> Rezumat din `polytabletop list`: Ruletă europeană (0–36) pentru doi: 100 de jetoane fiecare, 12 învârtiri, un singur pariu pe învârtire (5, 10 sau 20). Număr plin 35:1, roșu/negru, par/impar, mic/mare 1:1, duzine și coloane 2:1. Mutări: rosu 10, plin 17 5, duz2 20, invart.

## Scop

După 12 învârtiri ai mai multe jetoane decât adversarul.

## Pregătire

Fiecare jucător are 100 de jetoane; roata europeană are 37 de numere (0–36).

## Desfășurarea turei

Pui un pariu (J1 întâi, J2 vede pariul), apoi J1 spune `invart` și bila cade; câștigurile se plătesc din stocul tău. Zero pierde toate pariurile (fără „la partage”).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `rosu 10`
* `plin 17 5`
* `invart`

## Simplificări și note

Învârtirea este o mutare separată, ca AI-ul să nu vadă rezultatul; AI-ul alege pariuri după speranța matematică (aceeași pentru toate), deci joacă la întâmplare.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
