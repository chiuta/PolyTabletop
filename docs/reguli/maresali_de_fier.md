# Mareșali de Fier (`maresali_de_fier`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play maresali_de_fier --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 12×8 pătrată (8 direcții) cu stâlpi și adăposturi împrăștiate. Puncte de acțiune: 2 plus 1 pentru fiecare lider viu (maxim 5); fiecare acțiune costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Regiment de bătaie 4/3/1/3/4; Mașini de asediu 3/1/7/4/4; Mareșal 3/6/1/2/4; Călăreți de oțel 4/6/1/3/5. J1: 2× Regiment de bătaie, Mașini de asediu, Mareșal, 2× Călăreți de oțel. J2: 2× Regiment de bătaie, Mașini de asediu, Mareșal, 2× Călăreți de oțel. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri. Câștigi doborând liderul inamic. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «warmaster».

## Reguli

Variantă originală pentru doi pe o tablă 12×8 pătrată (8 direcții) cu stâlpi și adăposturi împrăștiate. Puncte de acțiune: 2 plus 1 pentru fiecare lider viu (maxim 5); fiecare acțiune costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Regiment de bătaie 4/3/1/3/4; Mașini de asediu 3/1/7/4/4; Mareșal 3/6/1/2/4; Călăreți de oțel 4/6/1/3/5. J1: 2× Regiment de bătaie, Mașini de asediu, Mareșal, 2× Călăreți de oțel. J2: 2× Regiment de bătaie, Mașini de asediu, Mareșal, 2× Călăreți de oțel. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri. Câștigi doborând liderul inamic. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
