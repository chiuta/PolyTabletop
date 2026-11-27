# Aventurieri de Foileton (`aventurieri_de_foileton`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play aventurieri_de_foileton --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 8×8 pătrată (8 direcții) cu ruine (pereți și adăpost). La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Eroul de serie 4/5/3/3/4; Asistentul isteț 2/6/3/2/3; Atletul 3/5/1/3/4; Cercetașa curajoasă 2/4/4/2/3; Ucigaș cu mantie 3/4/1/3/4. J1: Eroul de serie, Asistentul isteț, Atletul. J2: Ucigaș cu mantie, Cercetașa curajoasă, Atletul, Asistentul isteț. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: împingere (2+ pagube în corp la corp împing ținta); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri. Adu jetoanele $ înapoi în primele tale două rânduri; primul la 4 câștigă. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «pulp alley».

## Reguli

Variantă originală pentru doi pe o tablă 8×8 pătrată (8 direcții) cu ruine (pereți și adăpost). La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Eroul de serie 4/5/3/3/4; Asistentul isteț 2/6/3/2/3; Atletul 3/5/1/3/4; Cercetașa curajoasă 2/4/4/2/3; Ucigaș cu mantie 3/4/1/3/4. J1: Eroul de serie, Asistentul isteț, Atletul. J2: Ucigaș cu mantie, Cercetașa curajoasă, Atletul, Asistentul isteț. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: împingere (2+ pagube în corp la corp împing ținta); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri. Adu jetoanele $ înapoi în primele tale două rânduri; primul la 4 câștigă. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
