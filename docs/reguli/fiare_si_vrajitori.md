# Fiare și Vrăjitori (`fiare_si_vrajitori`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play fiare_si_vrajitori --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 9×8 hexagonală cu dărâmături (adăpost) și câțiva pereți. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Vrăjitor al turmei 5/3/4/3/3; Fiară cu coarne 6/4/1/3/5; Lup de stâncă 3/6/1/2/3; Șaman vindecător 2/3/1/1/3; Bătăuș de turmă 3/3/1/2/4. J1: Vrăjitor al turmei, Fiară cu coarne, Lup de stâncă, Șaman vindecător, Bătăuș de turmă. J2: Vrăjitor al turmei, Fiară cu coarne, Lup de stâncă, Șaman vindecător, Bătăuș de turmă. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); împingere (2+ pagube în corp la corp împing ținta). Unități speciale: lideri; atacuri în arie (1 pagubă în jurul țintei, și aliaților); vindecători (h); armură (cu 1 pagubă mai puțin). Câștigi doborând liderul inamic. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «hordes».

## Reguli

Variantă originală pentru doi pe o tablă 9×8 hexagonală cu dărâmături (adăpost) și câțiva pereți. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Vrăjitor al turmei 5/3/4/3/3; Fiară cu coarne 6/4/1/3/5; Lup de stâncă 3/6/1/2/3; Șaman vindecător 2/3/1/1/3; Bătăuș de turmă 3/3/1/2/4. J1: Vrăjitor al turmei, Fiară cu coarne, Lup de stâncă, Șaman vindecător, Bătăuș de turmă. J2: Vrăjitor al turmei, Fiară cu coarne, Lup de stâncă, Șaman vindecător, Bătăuș de turmă. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); împingere (2+ pagube în corp la corp împing ținta). Unități speciale: lideri; atacuri în arie (1 pagubă în jurul țintei, și aliaților); vindecători (h); armură (cu 1 pagubă mai puțin). Câștigi doborând liderul inamic. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
