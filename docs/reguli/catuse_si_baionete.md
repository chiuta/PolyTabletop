# Cartușe și Baionete (`catuse_si_baionete`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play catuse_si_baionete --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 10×8 pătrată (8 direcții) cu teren deschis. Puncte de acțiune: 2 plus 1 pentru fiecare lider viu (maxim 5); fiecare acțiune costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Soldat de linie 2/3/5/2/4; Grenadier 3/3/1/3/4; Ofițer 3/4/3/2/4; Tobosar 2/4/1/1/3. J1: 3× Soldat de linie, Grenadier, Ofițer, Tobosar. J2: 3× Soldat de linie, Grenadier, Ofițer, Tobosar. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: moral (șocul slăbește unitățile; r reface moralul); asalt (+1 zar după o mutare de 2+ pătrate); zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri; vindecători (h). Câștigi distrugând toate unitățile inamice. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, r uN, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «sharp practice».

## Reguli

Variantă originală pentru doi pe o tablă 10×8 pătrată (8 direcții) cu teren deschis. Puncte de acțiune: 2 plus 1 pentru fiecare lider viu (maxim 5); fiecare acțiune costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Soldat de linie 2/3/5/2/4; Grenadier 3/3/1/3/4; Ofițer 3/4/3/2/4; Tobosar 2/4/1/1/3. J1: 3× Soldat de linie, Grenadier, Ofițer, Tobosar. J2: 3× Soldat de linie, Grenadier, Ofițer, Tobosar. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: moral (șocul slăbește unitățile; r reface moralul); asalt (+1 zar după o mutare de 2+ pătrate); zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri; vindecători (h). Câștigi distrugând toate unitățile inamice. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, r uN, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
