# Darurile Serii (duel cu cărți ascunse) (`darurile_serii`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play darurile_serii --seed 1`. Sursa implementării: `polytabletop-games/src/darurile_serii.rs`.

> Rezumat din `polytabletop list`: Șase daruri (valori 2,2,3,3,5,6), câte o carte pentru fiecare punct de valoare. În fiecare rundă faci o dată fiecare acțiune: «ascund c» (1 carte ascunsă, numărată la final), «arunc a b» (2 cărți ies din joc), «ofer a b c» (arăți 3: adversarul ia una cu «ia c», tu păstrezi două), «impart a b + c d» (două perechi: adversarul alege «pereche 1|2»). La final fiecare dar merge la cine are mai multe cărți la el. Câștigi cu 11 puncte sau 4 daruri; după 6 runde decide scorul. Informație ascunsă (HOTSEAT_SAFE).

## Despre variantă

Joc original, scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «hanamikoji».

## Reguli

Șase daruri (valori 2,2,3,3,5,6), câte o carte pentru fiecare punct de valoare. În fiecare rundă faci o dată fiecare acțiune: «ascund c» (1 carte ascunsă, numărată la final), «arunc a b» (2 cărți ies din joc), «ofer a b c» (arăți 3: adversarul ia una cu «ia c», tu păstrezi două), «impart a b + c d» (două perechi: adversarul alege «pereche 1|2»). La final fiecare dar merge la cine are mai multe cărți la el. Câștigi cu 11 puncte sau 4 daruri; după 6 runde decide scorul. Informație ascunsă (HOTSEAT_SAFE).

## Informație ascunsă și AI

Mâna proprie și cărțile ascunse se văd doar pe rândul jucătorului (HOTSEAT_SAFE); AI-ul evaluează doar ce vede. Nivelurile Easy/Normal/Hard/Expert scalează adâncimea/timpul de căutare.
