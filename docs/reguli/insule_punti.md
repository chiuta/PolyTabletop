# Insulele Punților (poduri între insule) (`insule_punti`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play insule_punti --seed 1`. Sursa implementării: `polytabletop-games/src/insule_punti.rs`.

> Rezumat din `polytabletop list`: Douăsprezece insule legate prin 16 rute. Ai 3 cărți deschise (fiecare e o insulă). Pe rând joci o carte: «pun a b» (pod pe ruta a-b care atinge insula cărții), «tai a b» (tai un pod al adversarului pe o rută care atinge insula cărții) sau «descarc a» (schimbi cartea). La mutările 12, 24 și 36 se numără: insula e a celui cu mai multe poduri la ea (marcajul rămâne la egalitate), fiecare insulă marcată = 1 punct; apoi podurile se retrag. Câștigă cine are mai multe puncte.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «kahuna».

## Reguli

Douăsprezece insule legate prin 16 rute. Ai 3 cărți deschise (fiecare e o insulă). Pe rând joci o carte: «pun a b» (pod pe ruta a-b care atinge insula cărții), «tai a b» (tai un pod al adversarului pe o rută care atinge insula cărții) sau «descarc a» (schimbi cartea). La mutările 12, 24 și 36 se numără: insula e a celui cu mai multe poduri la ea (marcajul rămâne la egalitate), fiecare insulă marcată = 1 punct; apoi podurile se retrag. Câștigă cine are mai multe puncte.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
