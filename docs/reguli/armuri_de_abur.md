# Armuri de Abur (`armuri_de_abur`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play armuri_de_abur --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 9×9 pătrată (8 direcții) cu ruine (pereți și adăpost). La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Trăgător cu abur 2/4/5/2/4; Mers de oțel 6/3/5/3/5; Căpitan de abur 4/4/3/3/4; Grenadier cu flacără 3/3/3/2/3. J1: 2× Trăgător cu abur, Mers de oțel, Căpitan de abur, Grenadier cu flacără. J2: 2× Trăgător cu abur, Mers de oțel, Căpitan de abur, Grenadier cu flacără. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: imobilizare (focul la distanță încetinește ținta). Unități speciale: lideri; atacuri în arie (1 pagubă în jurul țintei, și aliaților); armură (cu 1 pagubă mai puțin). Câștigi distrugând toate unitățile inamice. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «konflikt '47».

## Reguli

Variantă originală pentru doi pe o tablă 9×9 pătrată (8 direcții) cu ruine (pereți și adăpost). La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Trăgător cu abur 2/4/5/2/4; Mers de oțel 6/3/5/3/5; Căpitan de abur 4/4/3/3/4; Grenadier cu flacără 3/3/3/2/3. J1: 2× Trăgător cu abur, Mers de oțel, Căpitan de abur, Grenadier cu flacără. J2: 2× Trăgător cu abur, Mers de oțel, Căpitan de abur, Grenadier cu flacără. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: imobilizare (focul la distanță încetinește ținta). Unități speciale: lideri; atacuri în arie (1 pagubă în jurul țintei, și aliaților); armură (cu 1 pagubă mai puțin). Câștigi distrugând toate unitățile inamice. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
