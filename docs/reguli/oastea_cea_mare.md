# Oastea cea Mare (`oastea_cea_mare`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play oastea_cea_mare --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 12×9 pătrată (4 direcții) cu o creastă centrală cu treceri. La fiecare tură un zar de comandă dă 2–4 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Pedestrime de linie 3/3/1/3/4; Pedestrime ușoară 2/4/3/2/3; Cavalerie de elită 4/6/1/3/5; General de oaste 4/5/1/3/4. J1: 3× Pedestrime de linie, 2× Pedestrime ușoară, Cavalerie de elită, General de oaste. J2: 3× Pedestrime de linie, 2× Pedestrime ușoară, Cavalerie de elită, General de oaste. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă); moral (șocul slăbește unitățile; r reface moralul); zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri. Câștigi doborând liderul inamic. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «de bellis multitudinis».

## Reguli

Variantă originală pentru doi pe o tablă 12×9 pătrată (4 direcții) cu o creastă centrală cu treceri. La fiecare tură un zar de comandă dă 2–4 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Pedestrime de linie 3/3/1/3/4; Pedestrime ușoară 2/4/3/2/3; Cavalerie de elită 4/6/1/3/5; General de oaste 4/5/1/3/4. J1: 3× Pedestrime de linie, 2× Pedestrime ușoară, Cavalerie de elită, General de oaste. J2: 3× Pedestrime de linie, 2× Pedestrime ușoară, Cavalerie de elită, General de oaste. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă); moral (șocul slăbește unitățile; r reface moralul); zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri. Câștigi doborând liderul inamic. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
