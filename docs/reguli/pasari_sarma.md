# Păsări pe Sârmă (colectare de culori) (`pasari_sarma`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pasari_sarma --seed 1`. Sursa implementării: `polytabletop-games/src/pasari_sarma.rs`.

> Rezumat din `polytabletop list`: Pe o sârmă comună stau păsări de 5 culori (r, v, a, g, n). Ai o mână deschisă de 5 cărți. La rând așezi o pasăre la capătul stâng sau drept («st r», «dr v»). Dacă la acel capăt se formează un șir de cel puțin 2 păsări la fel, le iei pe toate în colecție. Apoi tragi o carte la întâmplare (pachet de 40). La final, fiecare culoare colectată valorează 1+2+…+n puncte pentru n păsări. Câștigă cine are mai multe puncte.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «cubirds».

## Reguli

Pe o sârmă comună stau păsări de 5 culori (r, v, a, g, n). Ai o mână deschisă de 5 cărți. La rând așezi o pasăre la capătul stâng sau drept («st r», «dr v»). Dacă la acel capăt se formează un șir de cel puțin 2 păsări la fel, le iei pe toate în colecție. Apoi tragi o carte la întâmplare (pachet de 40). La final, fiecare culoare colectată valorează 1+2+…+n puncte pentru n păsări. Câștigă cine are mai multe puncte.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
