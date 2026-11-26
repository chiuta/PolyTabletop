# Piatră, hârtie, foarfece (`rps`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play rps --seed 1`. Sursa implementării: `polytabletop-games/src/rps.rs`.

> Rezumat din `polytabletop list`: Piatra sparge foarfeca, foarfeca taie hârtia, hârtia acoperă piatra. Amândoi aleg în secret, apoi arată; primul la 5 runde câștigate (maximum 15 runde, egalitățile se reiau). Mutări: piatra, hartie, foarfeca, arat.

## Scop

Primul la 5 runde câștigate (sau mai multe victorii după 15 runde).

## Pregătire

Alegerile sunt secrete; J1 alege, apoi J2, apoi J1 spune `arat`.

## Desfășurarea turei

`piatra`, `hartie` sau `foarfeca`, apoi `arat` (mutare separată): se compară alegerile.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `piatra`
* `hartie`
* `arat`

## Simplificări și note

Informația ascunsă: alegerea fiecăruia (hot-seat arată doar a celui la mutare). AI-ul nu o vede: contrează frecvența alegerilor tale din rundele arătate.

## Informație ascunsă

Jocul are informație ascunsă: `render` arată doar ce are voie să vadă jucătorul care privește (în hot-seat, doar jucătorul la mutare), iar AI-ul nu citește informația ascunsă a adversarului.
