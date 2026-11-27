# Suliți și Archebuze (`sulite_si_archebuze`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sulite_si_archebuze --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 12×8 pătrată (8 direcții) cu teren deschis. Punctele de acțiune sunt jumătate din unitățile tale vii (2–5); fiecare acțiune costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Pichier 4/2/1/3/5; Archebuzier 3/3/4/3/4; Cavalerie de oțel 4/6/1/3/5; Colonel 3/5/1/2/4. J1: 2× Pichier, 2× Archebuzier, Cavalerie de oțel, Colonel. J2: 2× Pichier, 2× Archebuzier, Cavalerie de oțel, Colonel. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă); zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri; armură (cu 1 pagubă mai puțin). Câștigi doborând liderul inamic. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «pike & shotte».

## Reguli

Variantă originală pentru doi pe o tablă 12×8 pătrată (8 direcții) cu teren deschis. Punctele de acțiune sunt jumătate din unitățile tale vii (2–5); fiecare acțiune costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Pichier 4/2/1/3/5; Archebuzier 3/3/4/3/4; Cavalerie de oțel 4/6/1/3/5; Colonel 3/5/1/2/4. J1: 2× Pichier, 2× Archebuzier, Cavalerie de oțel, Colonel. J2: 2× Pichier, 2× Archebuzier, Cavalerie de oțel, Colonel. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă); zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri; armură (cu 1 pagubă mai puțin). Câștigi doborând liderul inamic. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
