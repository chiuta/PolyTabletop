# Armata de Câmpie (`armata_de_cimpie`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play armata_de_cimpie --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 12×8 pătrată (8 direcții) cu teren deschis. Punctele de acțiune sunt jumătate din unitățile tale vii (2–5); fiecare acțiune costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Batalion de linie 4/2/4/3/4; Cavalerie ușoară 3/7/1/3/4; Artilerie de câmp 3/1/7/4/5; General 3/5/1/2/4. J1: 3× Batalion de linie, 2× Cavalerie ușoară, Artilerie de câmp, General. J2: 3× Batalion de linie, 2× Cavalerie ușoară, Artilerie de câmp, General. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă); moral (șocul slăbește unitățile; r reface moralul). Unități speciale: lideri. Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 9 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «lasalle».

## Reguli

Variantă originală pentru doi pe o tablă 12×8 pătrată (8 direcții) cu teren deschis. Punctele de acțiune sunt jumătate din unitățile tale vii (2–5); fiecare acțiune costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Batalion de linie 4/2/4/3/4; Cavalerie ușoară 3/7/1/3/4; Artilerie de câmp 3/1/7/4/5; General 3/5/1/2/4. J1: 3× Batalion de linie, 2× Cavalerie ușoară, Artilerie de câmp, General. J2: 3× Batalion de linie, 2× Cavalerie ușoară, Artilerie de câmp, General. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă); moral (șocul slăbește unitățile; r reface moralul). Unități speciale: lideri. Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 9 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
