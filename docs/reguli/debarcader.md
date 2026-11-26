# Debarcaderul Norocos (risc la port) (`debarcader`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play debarcader --seed 1`. Sursa implementării: `polytabletop-games/src/debarcader.rs`.

> Rezumat din `polytabletop list`: Joc de risc la un port. La rând tragi bărci una câte una («trag»): fiecare are un pavilion (4 feluri) și o încărcătură de 1–3 monede. Dacă apare al doilea pavilion de același fel în tura asta, barca se scufundă: pierzi tot ce ai tras și tura se termină. După cel puțin o barcă poți să te oprești («opresc») și să încasezi încărcăturile plus 1 monedă pe dulgher angajat. La începutul turei, cu 6 monede, poți angaja un dulgher («angajez»: +1 influență). Câștigă primul cu 4 influențe.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «port royal».

## Reguli

Joc de risc la un port. La rând tragi bărci una câte una («trag»): fiecare are un pavilion (4 feluri) și o încărcătură de 1–3 monede. Dacă apare al doilea pavilion de același fel în tura asta, barca se scufundă: pierzi tot ce ai tras și tura se termină. După cel puțin o barcă poți să te oprești («opresc») și să încasezi încărcăturile plus 1 monedă pe dulgher angajat. La începutul turei, cu 6 monede, poți angaja un dulgher («angajez»: +1 influență). Câștigă primul cu 4 influențe.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
