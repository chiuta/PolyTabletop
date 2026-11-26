# Fronturile Mari (război pe hartă) (`fronturi_mari`)

Categorie: Strategie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play fronturi_mari --seed 1`. Sursa implementării: `polytabletop-games/src/brand_war.rs`.

> Rezumat din `polytabletop list`: Hartă de 12 teritorii (grilă 3×4) cu valori 1–3; fiecare are 3 teritorii-casă și 6 neutre. La începutul turei primești unități egal cu jumătate din venit (suma valorilor teritoriilor tale, minim 1) și le plasezi într-un teritoriu al tău («pun n»); apoi până la 3 mutări «mut a b toti|unu» către un vecin (la cel inamic sau neutru e atac: pierderi egale, atacatorul câștigă dacă are mai multe unități) și «gata». Câștigi luând capitala adversarului (teritoriul 2 sau 11); după 60 de ture decide scorul.

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «axis & allies».

## Reguli

Hartă de 12 teritorii (grilă 3×4) cu valori 1–3; fiecare are 3 teritorii-casă și 6 neutre. La începutul turei primești unități egal cu jumătate din venit (suma valorilor teritoriilor tale, minim 1) și le plasezi într-un teritoriu al tău («pun n»); apoi până la 3 mutări «mut a b toti|unu» către un vecin (la cel inamic sau neutru e atac: pierderi egale, atacatorul câștigă dacă are mai multe unități) și «gata». Câștigi luând capitala adversarului (teritoriul 2 sau 11); după 60 de ture decide scorul.

## Informație ascunsă și AI

Informația este deschisă; cărțile/piesele noi se trag la întâmplare cu un zar re-însămânțat la clonare, deci AI-ul nu vede viitorul și nu trișează. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
