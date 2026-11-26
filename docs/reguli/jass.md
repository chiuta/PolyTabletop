# Jass (Schieber) (`jass`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play jass --seed 1`. Sursa implementării: `polytabletop-games/src/q5_trick.rs, jass.rs`.

> Rezumat din `polytabletop list`: Jass elvețian pentru doi, pachet de 36 de cărți, 9 cărți fiecare, 4 mâini: cel care conduce alege atuul (atu h) sau îl dă mai departe (schiebe). Valetul de atu 20, noua 14, asul 11; ultima levată +5, match +100. Mutări: atu c, schiebe, 10h, Js.

## Scop

Aduni mai multe puncte de carte pe cele 4 mâini.

## Pregătire

9 cărți fiecare (celelalte 18 nu intră în joc); atuul se alege la începutul mâinii.

## Desfășurarea turei

Cel care conduce spune `atu h/d/s/c` sau `schiebe`; apoi joci cărți (`10h`, `Js`); răspunzi la culoare dacă poți, cu atu poți tăia oricând fără culoare.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `atu h`
* `10h`
* `Js`

## Simplificări și note

Adaptare pentru doi: fără Stöck, fără Weis și fără interdicția sub-tăierii. AI-ul nu vede mâna adversarului; hot-seat arată doar mâna celui la mutare.

## Informație ascunsă

Jocul are informație ascunsă: `render` arată doar ce are voie să vadă jucătorul care privește (în hot-seat, doar jucătorul la mutare), iar AI-ul nu citește informația ascunsă a adversarului.
