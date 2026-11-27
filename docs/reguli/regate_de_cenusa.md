# Regate de Cenușă (`regate_de_cenusa`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play regate_de_cenusa --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 8×8 pătrată (8 direcții) cu o creastă centrală cu treceri. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Rege-Cenușă 5/3/1/3/5; Gardian cu scut 4/2/1/3/5; Arcaș de ceață 2/3/5/2/3; Vrăjitor al focului 2/3/4/2/3; Lăncier 3/3/1/2/4. J1: Rege-Cenușă, 2× Gardian cu scut, Arcaș de ceață, Vrăjitor al focului. J2: Rege-Cenușă, 2× Gardian cu scut, Arcaș de ceață, Vrăjitor al focului. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri; atacuri în arie (1 pagubă în jurul țintei, și aliaților); armură (cu 1 pagubă mai puțin). Câștigi doborând liderul inamic. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «wrath of kings».

## Reguli

Variantă originală pentru doi pe o tablă 8×8 pătrată (8 direcții) cu o creastă centrală cu treceri. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Rege-Cenușă 5/3/1/3/5; Gardian cu scut 4/2/1/3/5; Arcaș de ceață 2/3/5/2/3; Vrăjitor al focului 2/3/4/2/3; Lăncier 3/3/1/2/4. J1: Rege-Cenușă, 2× Gardian cu scut, Arcaș de ceață, Vrăjitor al focului. J2: Rege-Cenușă, 2× Gardian cu scut, Arcaș de ceață, Vrăjitor al focului. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri; atacuri în arie (1 pagubă în jurul țintei, și aliaților); armură (cu 1 pagubă mai puțin). Câștigi doborând liderul inamic. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
