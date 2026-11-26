# Spoons (lingurile) (`spoons`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play spoons --seed 1`. Sursa implementării: `polytabletop-games/src/spoons.rs`.

> Rezumat din `polytabletop list`: Joc de petrecere cu cărți pentru doi, cu o singură lingură: tragi o carte, arunci una, strângi patru la fel și iei lingura; cine pierde primește o literă din SPOONS. 12 runde, câștigă cine are mai puține litere. Mutări: iau, dau K, lingura, mai-departe.

## Scop

Ai mai puține litere din SPOONS după 12 runde (cine le strânge pe toate pierde).

## Pregătire

Pachet de 52 de cărți (doar rangul contează), 4 cărți fiecare, o singură lingură în mijloc; dealer-ul alternează.

## Desfășurarea turei

`iau` (tragi), apoi `dau 7` (arunci o carte de acel rang) sau `lingura` dacă ai patru la fel; cine ia lingura câștigă runda; `mai-departe` începe runda următoare.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `iau`
* `dau K`
* `lingura`

## Simplificări și note

Simplificare: fără reacția în timp real; lingura se ia doar cu patru la fel. Informația ascunsă (mâna adversarului, pachetul) nu e citită de AI; hot-seat arată doar mâna celui la mutare.

## Informație ascunsă

Jocul are informație ascunsă: `render` arată doar ce are voie să vadă jucătorul care privește (în hot-seat, doar jucătorul la mutare), iar AI-ul nu citește informația ascunsă a adversarului.
