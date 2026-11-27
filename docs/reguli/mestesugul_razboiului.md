# Meșteșugul Războiului (`mestesugul_razboiului`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mestesugul_razboiului --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 10×8 pătrată (8 direcții) cu o creastă centrală cu treceri. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Infanterie de elită 4/3/1/3/5; Arcași ușori 2/4/4/2/3; Cavalerie de război 3/6/1/3/4; Strateg 3/4/1/3/4. J1: 2× Infanterie de elită, 2× Arcași ușori, Cavalerie de război, Strateg. J2: 2× Infanterie de elită, 2× Arcași ușori, Cavalerie de război, Strateg. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: flancare (+1 zar dacă un aliat e lângă țintă); asalt (+1 zar după o mutare de 2+ pătrate). Unități speciale: lideri. Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 8 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «l'art de la guerre».

## Reguli

Variantă originală pentru doi pe o tablă 10×8 pătrată (8 direcții) cu o creastă centrală cu treceri. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Infanterie de elită 4/3/1/3/5; Arcași ușori 2/4/4/2/3; Cavalerie de război 3/6/1/3/4; Strateg 3/4/1/3/4. J1: 2× Infanterie de elită, 2× Arcași ușori, Cavalerie de război, Strateg. J2: 2× Infanterie de elită, 2× Arcași ușori, Cavalerie de război, Strateg. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: flancare (+1 zar dacă un aliat e lângă țintă); asalt (+1 zar după o mutare de 2+ pătrate). Unități speciale: lideri. Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 8 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
