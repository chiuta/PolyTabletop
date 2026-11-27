# Legământ de Piatră (`legamant_de_piatra`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play legamant_de_piatra --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 10×8 pătrată (8 direcții) cu o creastă centrală cu treceri. Punctele de acțiune sunt jumătate din unitățile tale vii (2–5); fiecare acțiune costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Gardă de piatră 5/2/1/3/5; Arbaletier de munte 2/3/5/3/3; Thane de peșteră 5/3/1/3/5; Sapator 3/3/1/3/4; Orc de câmp 2/4/1/2/4. J1: 2× Gardă de piatră, 2× Arbaletier de munte, Thane de peșteră, Sapator. J2: 5× Orc de câmp, Sapator, Arbaletier de munte. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: moral (șocul slăbește unitățile; r reface moralul); asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri; armură (cu 1 pagubă mai puțin). Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 8 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «oathmark».

## Reguli

Variantă originală pentru doi pe o tablă 10×8 pătrată (8 direcții) cu o creastă centrală cu treceri. Punctele de acțiune sunt jumătate din unitățile tale vii (2–5); fiecare acțiune costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Gardă de piatră 5/2/1/3/5; Arbaletier de munte 2/3/5/3/3; Thane de peșteră 5/3/1/3/5; Sapator 3/3/1/3/4; Orc de câmp 2/4/1/2/4. J1: 2× Gardă de piatră, 2× Arbaletier de munte, Thane de peșteră, Sapator. J2: 5× Orc de câmp, Sapator, Arbaletier de munte. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: moral (șocul slăbește unitățile; r reface moralul); asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri; armură (cu 1 pagubă mai puțin). Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 8 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
