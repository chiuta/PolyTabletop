# Înălțimi cu Eroi (`inaltimi_cu_eroi`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play inaltimi_cu_eroi --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 9×9 hexagonală cu dealuri (înălțimi). La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Erou de vale 4/4/1/3/4; Arcaș de creastă 2/3/6/2/3; Titan de piatră 6/2/1/4/5; Iscoadă 2/6/1/2/3; Zburător 2/6/2/2/3. J1: Erou de vale, Arcaș de creastă, Titan de piatră, Iscoadă, Zburător. J2: Erou de vale, Arcaș de creastă, Titan de piatră, Iscoadă, Zburător. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: zburători (ignoră terenul); armură (cu 1 pagubă mai puțin). Câștigi distrugând toate unitățile inamice. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «heroscape».

## Reguli

Variantă originală pentru doi pe o tablă 9×9 hexagonală cu dealuri (înălțimi). La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Erou de vale 4/4/1/3/4; Arcaș de creastă 2/3/6/2/3; Titan de piatră 6/2/1/4/5; Iscoadă 2/6/1/2/3; Zburător 2/6/2/2/3. J1: Erou de vale, Arcaș de creastă, Titan de piatră, Iscoadă, Zburător. J2: Erou de vale, Arcaș de creastă, Titan de piatră, Iscoadă, Zburător. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: zburători (ignoră terenul); armură (cu 1 pagubă mai puțin). Câștigi distrugând toate unitățile inamice. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
