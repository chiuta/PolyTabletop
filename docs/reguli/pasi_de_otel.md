# Pași de Oțel (`pasi_de_otel`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pasi_de_otel --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 10×9 hexagonală cu stâlpi și adăposturi împrăștiate. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Mers ușor 4/5/5/2/5; Mers greu 6/3/6/3/6; Pușcaș în exoschelet 3/4/4/2/4; Comandant de mers 5/4/5/3/5. J1: Mers ușor, Mers greu, Pușcaș în exoschelet, Comandant de mers. J2: Mers ușor, Mers greu, Pușcaș în exoschelet, Comandant de mers. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri; armură (cu 1 pagubă mai puțin). Câștigi doborând liderul inamic. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «heavy gear».

## Reguli

Variantă originală pentru doi pe o tablă 10×9 hexagonală cu stâlpi și adăposturi împrăștiate. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Mers ușor 4/5/5/2/5; Mers greu 6/3/6/3/6; Pușcaș în exoschelet 3/4/4/2/4; Comandant de mers 5/4/5/3/5. J1: Mers ușor, Mers greu, Pușcaș în exoschelet, Comandant de mers. J2: Mers ușor, Mers greu, Pușcaș în exoschelet, Comandant de mers. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri; armură (cu 1 pagubă mai puțin). Câștigi doborând liderul inamic. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
