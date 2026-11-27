# Cavaleri și Pavaze (`cavaleri_si_pavaze`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cavaleri_si_pavaze --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 10×8 pătrată (8 direcții) cu teren deschis. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Cavaler greu 5/5/1/4/5; Pavăzar 4/2/1/2/6; Arbaletier cu pavăză 3/3/5/2/4; Senior 4/4/1/3/5. J1: Cavaler greu, 2× Pavăzar, Arbaletier cu pavăză, Senior. J2: Cavaler greu, 2× Pavăzar, Arbaletier cu pavăză, Senior. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); contraatac (supraviețuitorii din corp la corp lovesc înapoi). Unități speciale: lideri. Câștigi doborând liderul inamic. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «swordpoint».

## Reguli

Variantă originală pentru doi pe o tablă 10×8 pătrată (8 direcții) cu teren deschis. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Cavaler greu 5/5/1/4/5; Pavăzar 4/2/1/2/6; Arbaletier cu pavăză 3/3/5/2/4; Senior 4/4/1/3/5. J1: Cavaler greu, 2× Pavăzar, Arbaletier cu pavăză, Senior. J2: Cavaler greu, 2× Pavăzar, Arbaletier cu pavăză, Senior. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); contraatac (supraviețuitorii din corp la corp lovesc înapoi). Unități speciale: lideri. Câștigi doborând liderul inamic. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
