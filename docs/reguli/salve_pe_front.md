# Salve pe Front (`salve_pe_front`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play salve_pe_front --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 11×9 hexagonală cu teren deschis. Punctele de acțiune sunt jumătate din unitățile tale vii (2–5); fiecare acțiune costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Brigadă de linie 5/2/3/3/4; Brigadă de cavalerie 4/5/1/3/4; Baterie 3/1/6/4/4; Corp de armată 4/3/1/2/4. J1: 3× Brigadă de linie, Brigadă de cavalerie, Baterie, Corp de armată. J2: 3× Brigadă de linie, Brigadă de cavalerie, Baterie, Corp de armată. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: moral (șocul slăbește unitățile; r reface moralul); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri. Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 8 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «volley & bayonet».

## Reguli

Variantă originală pentru doi pe o tablă 11×9 hexagonală cu teren deschis. Punctele de acțiune sunt jumătate din unitățile tale vii (2–5); fiecare acțiune costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Brigadă de linie 5/2/3/3/4; Brigadă de cavalerie 4/5/1/3/4; Baterie 3/1/6/4/4; Corp de armată 4/3/1/2/4. J1: 3× Brigadă de linie, Brigadă de cavalerie, Baterie, Corp de armată. J2: 3× Brigadă de linie, Brigadă de cavalerie, Baterie, Corp de armată. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: moral (șocul slăbește unitățile; r reface moralul); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri. Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 8 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
