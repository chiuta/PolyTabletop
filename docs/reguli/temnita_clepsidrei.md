# Temnița Clepsidrei (`temnita_clepsidrei`)

Categorie: Cooperativ. Echipa (Jucătorul 1) contra Jocului (Jucătorul 2, mutări forțate). Pornire: `polytabletop play temnita_clepsidrei --seed 1`. Sursa implementării: `polytabletop-games/src/c2_siege.rs` (specificația în `c2_specs.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cooperativă de apărare: 3 culoare (coridor, sală, gârlă); echipa (J1: Cavalerul și Magiciana, câte 2 acțiuni pe rând) apără o bază cu 6 viață. Fiecare acțiune joacă o carte (1–5) din mână: atac L k lovește amenințarea cea mai apropiată de pe culoarul L cu valoarea cărții k (pe care personajul o poate spori cu un bonus propriu). J2 este Jocul: mutarea forțată eveniment înaintează amenințările, cele care ajung rup baza și apar altele noi (a cincea parte din ele mai tari pe parcurs). Înghețul (ingh) oprește un culoar o rundă. Câștigă echipa (J1) dacă ține baza 6 runde și doboară căpetenia; câștigă Jocul (J2) dacă baza cade sau timpul se depășește. Mutări: atac L k, ingh L k, pas.

Informație ascunsă: doar ordinea pachetelor, care nu aparține vreunui jucător (nu există informație privată).
