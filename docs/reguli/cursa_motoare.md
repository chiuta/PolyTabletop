# Cursa Motoarelor (cursă cu cărți) (`cursa_motoare`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cursa_motoare --seed 1`. Sursa implementării: `polytabletop-games/src/brand_race.rs`.

> Rezumat din `polytabletop list`: Cursă cu un bolid pe 40 de trepte și 4 curbe (la 8, 17, 26, 34). Ai 3 cărți deschise de viteză 1–5. Joci o carte («c1») sau o joci cu supraturare («c1+»: +2 viteză, +1 căldură). Dacă treci o curbă cu viteza peste 4, primești căldură egală cu depășirea; o carte de 1–2 fără supraturare răcește cu 1. La 6 căldură motorul cedează și pierzi. Aspirație: +1 dacă te oprești imediat în spatele adversarului. Primul la linie câștigă.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «heat: pedal to the metal».

## Reguli

Cursă cu un bolid pe 40 de trepte și 4 curbe (la 8, 17, 26, 34). Ai 3 cărți deschise de viteză 1–5. Joci o carte («c1») sau o joci cu supraturare («c1+»: +2 viteză, +1 căldură). Dacă treci o curbă cu viteza peste 4, primești căldură egală cu depășirea; o carte de 1–2 fără supraturare răcește cu 1. La 6 căldură motorul cedează și pierzi. Aspirație: +1 dacă te oprești imediat în spatele adversarului. Primul la linie câștigă.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
