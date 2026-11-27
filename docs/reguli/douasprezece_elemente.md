# Douăsprezece Elemente (`douasprezece_elemente`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play douasprezece_elemente --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 10×8 pătrată (4 direcții) cu teren deschis. La fiecare tură un zar de comandă dă 2–4 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Element de lănci 3/3/1/3/4; Element de arcași 2/3/4/2/3; Element de care 3/5/1/3/4; General 3/4/1/2/4. J1: 2× Element de lănci, 2× Element de arcași, Element de care, General. J2: 2× Element de lănci, 2× Element de arcași, Element de care, General. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă); zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri. Câștigi doborând liderul inamic. Cel mult 16 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «de bellis antiquitatis».

## Reguli

Variantă originală pentru doi pe o tablă 10×8 pătrată (4 direcții) cu teren deschis. La fiecare tură un zar de comandă dă 2–4 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Element de lănci 3/3/1/3/4; Element de arcași 2/3/4/2/3; Element de care 3/5/1/3/4; General 3/4/1/2/4. J1: 2× Element de lănci, 2× Element de arcași, Element de care, General. J2: 2× Element de lănci, 2× Element de arcași, Element de care, General. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă); zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri. Câștigi doborând liderul inamic. Cel mult 16 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
