# Golemii din Atelier (motor de resurse) (`golemii_atelier`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play golemii_atelier --seed 1`. Sursa implementării: `polytabletop-games/src/brand_market.rs`.

> Rezumat din `polytabletop list`: Strângi resurse (R, V, A, G, N) și cumperi cărți din oferta de 5; cărțile au cost și puncte. Câștigă cine are primul 18 puncte (runda se încheie egal; la egalitate, mai puține cărți). Cel mult 12 resurse în mână. Mutări: «cumpar i», «las x» (arunci o resursă dacă nu poți altceva). «ia2 x»: două resurse la fel (sau «ia1 x» dacă ai doar un loc). Cărțile dau doar puncte. «schimb x»: 2 resurse x devin 1 din tipul următor.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «century: golem edition».

## Reguli

Strângi resurse (R, V, A, G, N) și cumperi cărți din oferta de 5; cărțile au cost și puncte. Câștigă cine are primul 18 puncte (runda se încheie egal; la egalitate, mai puține cărți). Cel mult 12 resurse în mână. Mutări: «cumpar i», «las x» (arunci o resursă dacă nu poți altceva). «ia2 x»: două resurse la fel (sau «ia1 x» dacă ai doar un loc). Cărțile dau doar puncte. «schimb x»: 2 resurse x devin 1 din tipul următor.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
