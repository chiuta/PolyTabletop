# Bile (par sau impar) (`marbles`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play marbles --seed 1`. Sursa implementării: `polytabletop-games/src/marbles.rs`.

> Rezumat din `polytabletop list`: Joc antic de ghicit: cel care ține ascunde 1–5 bile, celălalt ghicește par sau impar; ghicești bine = iei atâtea bile, rău = le dai. 10 bile fiecare, 12 runde, rolurile alternează. Mutări: ascund 3, par, impar, arat.

## Scop

După 12 runde ai mai multe bile (sau adversarul rămâne fără).

## Pregătire

Fiecare jucător are 10 bile; J1 ține primul.

## Desfășurarea turei

Cel care ține ascunde N bile (`ascund N`, 1–5), celălalt ghicește `par` sau `impar`, apoi cel care ține spune `arat`. Informația ascunsă e numărul de bile din pumn: `render` îl arată doar celui care ține.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `ascund 3`
* `par`
* `arat`

## Simplificări și note

AI-ul care ghicește nu citește numărul ascuns; cel care ascunde alege la întâmplare. Hot-seat: ecranul arată doar jucătorul la mutare.

## Informație ascunsă

Jocul are informație ascunsă: `render` arată doar ce are voie să vadă jucătorul care privește (în hot-seat, doar jucătorul la mutare), iar AI-ul nu citește informația ascunsă a adversarului.
