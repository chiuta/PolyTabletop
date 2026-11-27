# Trupe de Plumb (`trupe_de_plumb`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play trupe_de_plumb --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 10×8 pătrată (8 direcții) cu dărâmături (adăpost) și câțiva pereți. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Pușcaș 2/4/4/2/4; Mitralior 2/2/6/3/4; Tanc ușor 5/5/5/3/6; Sapator 2/4/1/2/3; Medic de front 2/3/1/1/3. J1: 2× Pușcaș, Mitralior, Tanc ușor, Sapator, Medic de front. J2: 2× Pușcaș, Mitralior, Tanc ușor, Sapator, Medic de front. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: imobilizare (focul la distanță încetinește ținta); moral (șocul slăbește unitățile; r reface moralul). Unități speciale: vindecători (h); armură (cu 1 pagubă mai puțin). Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 8 câștigă. Cel mult 10 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, r uN, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «axis & allies miniatures».

## Reguli

Variantă originală pentru doi pe o tablă 10×8 pătrată (8 direcții) cu dărâmături (adăpost) și câțiva pereți. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Pușcaș 2/4/4/2/4; Mitralior 2/2/6/3/4; Tanc ușor 5/5/5/3/6; Sapator 2/4/1/2/3; Medic de front 2/3/1/1/3. J1: 2× Pușcaș, Mitralior, Tanc ușor, Sapator, Medic de front. J2: 2× Pușcaș, Mitralior, Tanc ușor, Sapator, Medic de front. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: imobilizare (focul la distanță încetinește ținta); moral (șocul slăbește unitățile; r reface moralul). Unități speciale: vindecători (h); armură (cu 1 pagubă mai puțin). Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 8 câștigă. Cel mult 10 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, r uN, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
