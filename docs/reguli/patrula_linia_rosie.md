# Patrula de pe Linia Roșie (`patrula_linia_rosie`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play patrula_linia_rosie --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 9×9 pătrată (8 direcții) cu ruine (pereți și adăpost). La fiecare tură ai 4 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Pușcaș de patrulă 2/4/6/3/4; Lunetist 2/2/7/2/3; Sergent 3/4/5/3/4; Gealat de infiltrare 2/5/3/2/3. J1: 2× Pușcaș de patrulă, Lunetist, Sergent, Gealat de infiltrare. J2: 2× Pușcaș de patrulă, Lunetist, Sergent, Gealat de infiltrare. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: imobilizare (focul la distanță încetinește ținta). Unități speciale: lideri; trăgători mobili. Adu jetoanele $ înapoi în primele tale două rânduri; primul la 2 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «force on force».

## Reguli

Variantă originală pentru doi pe o tablă 9×9 pătrată (8 direcții) cu ruine (pereți și adăpost). La fiecare tură ai 4 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Pușcaș de patrulă 2/4/6/3/4; Lunetist 2/2/7/2/3; Sergent 3/4/5/3/4; Gealat de infiltrare 2/5/3/2/3. J1: 2× Pușcaș de patrulă, Lunetist, Sergent, Gealat de infiltrare. J2: 2× Pușcaș de patrulă, Lunetist, Sergent, Gealat de infiltrare. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: imobilizare (focul la distanță încetinește ținta). Unități speciale: lideri; trăgători mobili. Adu jetoanele $ înapoi în primele tale două rânduri; primul la 2 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
