# Luptă în Nori (`lupta_in_nori`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play lupta_in_nori --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 9×9 hexagonală cu teren deschis. La fiecare tură ai 2 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Biplan agil 2/7/2/3/4; Monoplan 3/6/3/3/4; Tunar de coadă 3/5/4/2/4; As 4/7/3/4/5. J1: Biplan agil, Monoplan, Tunar de coadă, As. J2: Biplan agil, Monoplan, Tunar de coadă, As. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: flancare (+1 zar dacă un aliat e lângă țintă); moral (șocul slăbește unitățile; r reface moralul). Unități speciale: lideri; zburători (ignoră terenul). Câștigi distrugând toate unitățile inamice. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «check your 6!».

## Reguli

Variantă originală pentru doi pe o tablă 9×9 hexagonală cu teren deschis. La fiecare tură ai 2 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Biplan agil 2/7/2/3/4; Monoplan 3/6/3/3/4; Tunar de coadă 3/5/4/2/4; As 4/7/3/4/5. J1: Biplan agil, Monoplan, Tunar de coadă, As. J2: Biplan agil, Monoplan, Tunar de coadă, As. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: flancare (+1 zar dacă un aliat e lângă țintă); moral (șocul slăbește unitățile; r reface moralul). Unități speciale: lideri; zburători (ignoră terenul). Câștigi distrugând toate unitățile inamice. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
