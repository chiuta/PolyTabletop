# Cadranul Ostașului (`cadranul_ostasului`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cadranul_ostasului --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 9×8 hexagonală cu dealuri (înălțimi). La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Cavaler de cadran 6/4/1/4/5; Arcaș de cadran 4/3/5/3/3; Vrăjitor de cadran 4/3/4/3/3; Scutier 3/4/1/2/4. J1: Cavaler de cadran, Arcaș de cadran, Vrăjitor de cadran, Scutier. J2: Cavaler de cadran, Arcaș de cadran, Vrăjitor de cadran, Scutier. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: atacuri în arie (1 pagubă în jurul țintei, și aliaților); cadrane (mai puține zaruri pe măsură ce scade viața). Câștigi distrugând toate unitățile inamice. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «mage knight».

## Reguli

Variantă originală pentru doi pe o tablă 9×8 hexagonală cu dealuri (înălțimi). La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Cavaler de cadran 6/4/1/4/5; Arcaș de cadran 4/3/5/3/3; Vrăjitor de cadran 4/3/4/3/3; Scutier 3/4/1/2/4. J1: Cavaler de cadran, Arcaș de cadran, Vrăjitor de cadran, Scutier. J2: Cavaler de cadran, Arcaș de cadran, Vrăjitor de cadran, Scutier. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: atacuri în arie (1 pagubă în jurul țintei, și aliaților); cadrane (mai puține zaruri pe măsură ce scade viața). Câștigi distrugând toate unitățile inamice. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
