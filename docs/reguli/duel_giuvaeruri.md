# Duelul Giuvaerelor (motor de resurse) (`duel_giuvaeruri`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play duel_giuvaeruri --seed 1`. Sursa implementării: `polytabletop-games/src/brand_market.rs`.

> Rezumat din `polytabletop list`: Strângi resurse (R, V, A, G) și cumperi cărți din oferta de 4; cărțile au cost și puncte. Câștigă cine are primul 14 puncte (runda se încheie egal; la egalitate, mai puține cărți). Cel mult 10 resurse în mână. Mutări: «cumpar i», «las x» (arunci o resursă dacă nu poți altceva). «ia rva»: trei resurse de feluri diferite. Unele cărți dau o reducere permanentă la un fel de resursă.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «splendor duel».

## Reguli

Strângi resurse (R, V, A, G) și cumperi cărți din oferta de 4; cărțile au cost și puncte. Câștigă cine are primul 14 puncte (runda se încheie egal; la egalitate, mai puține cărți). Cel mult 10 resurse în mână. Mutări: «cumpar i», «las x» (arunci o resursă dacă nu poți altceva). «ia rva»: trei resurse de feluri diferite. Unele cărți dau o reducere permanentă la un fel de resursă.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
