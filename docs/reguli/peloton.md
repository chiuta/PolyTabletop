# Pelotonul (cursă cu cărți) (`peloton`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play peloton --seed 1`. Sursa implementării: `polytabletop-games/src/brand_race.rs`.

> Rezumat din `polytabletop list`: Cursă pe 44 de trepte; fiecare are doi alergători: sprinterul (s: viteze 2–9, explozive) și rulorul (r: 3–7, regulat), fiecare cu 3 cărți deschise. Joci o carte și avansezi atâtea trepte; dacă te oprești imediat în spatele cuiva primești +1 (aspirație); dacă ajungi singur în față cu o carte ≥ 5, adaugi oboseală (mai multe cărți de 2 în pachet). Primul care trece linia câștigă. Mutare: «s1» (sprinter, cartea 1), «r2» (rulor, cartea 2).

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «flamme rouge».

## Reguli

Cursă pe 44 de trepte; fiecare are doi alergători: sprinterul (s: viteze 2–9, explozive) și rulorul (r: 3–7, regulat), fiecare cu 3 cărți deschise. Joci o carte și avansezi atâtea trepte; dacă te oprești imediat în spatele cuiva primești +1 (aspirație); dacă ajungi singur în față cu o carte ≥ 5, adaugi oboseală (mai multe cărți de 2 în pachet). Primul care trece linia câștigă. Mutare: «s1» (sprinter, cartea 1), «r2» (rulor, cartea 2).

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
