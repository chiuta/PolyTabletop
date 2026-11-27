# Apărarea Planetei (`apararea_planetei`)

Categorie: Cooperativ. Echipa (Jucătorul 1) contra Jocului (Jucătorul 2, mutări forțate). Pornire: `polytabletop play apararea_planetei --seed 1`. Sursa implementării: `polytabletop-games/src/c2_siege.rs` (specificația în `c2_specs.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cooperativă de apărare: 5 culoare (America, Europa, Asia, Africa, Oceania); echipa (J1: Comandantul și Cercetașa, câte 2 acțiuni pe rând) apără o bază cu 11 viață. Fiecare acțiune joacă o carte (1–5) din mână: atac L k lovește amenințarea cea mai apropiată de pe culoarul L cu valoarea cărții k (pe care personajul o poate spori cu un bonus propriu). J2 este Jocul: mutarea forțată eveniment înaintează amenințările, cele care ajung rup baza și apar altele noi (a cincea parte din ele mai tari pe parcurs). Zidul (zid) absorb lovituri pe culoar. Înghețul (ingh) oprește un culoar o rundă. Câștigă echipa (J1) dacă elimină 14 amenințări; câștigă Jocul (J2) dacă baza cade sau timpul se depășește. Mutări: atac L k, zid L k, ingh L k, pas.

Informație ascunsă: doar ordinea pachetelor, care nu aparține vreunui jucător (nu există informație privată).
