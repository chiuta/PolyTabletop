# Escadrila Cerului Roșu (`escadrila_cerului_rosu`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play escadrila_cerului_rosu --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 10×10 hexagonală cu teren deschis. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Vânător 3/6/3/3/4; Bombardier 5/4/2/4/5; Escortă 2/7/3/2/3; Comandant de escadrilă 4/6/3/3/4. J1: 2× Vânător, Bombardier, Comandant de escadrilă, Escortă. J2: 2× Vânător, Bombardier, Comandant de escadrilă, Escortă. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Unități speciale: lideri; zburători (ignoră terenul); atacuri în arie (1 pagubă în jurul țintei, și aliaților); trăgători mobili. Câștigi doborând liderul inamic. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «aeronautica imperialis».

## Reguli

Variantă originală pentru doi pe o tablă 10×10 hexagonală cu teren deschis. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Vânător 3/6/3/3/4; Bombardier 5/4/2/4/5; Escortă 2/7/3/2/3; Comandant de escadrilă 4/6/3/3/4. J1: 2× Vânător, Bombardier, Comandant de escadrilă, Escortă. J2: 2× Vânător, Bombardier, Comandant de escadrilă, Escortă. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Unități speciale: lideri; zburători (ignoră terenul); atacuri în arie (1 pagubă în jurul țintei, și aliaților); trăgători mobili. Câștigi doborând liderul inamic. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
