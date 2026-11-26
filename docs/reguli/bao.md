# Bao (Bao la Kiswahili) (`bao`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play bao --seed 1`. Sursa implementării: `polytabletop-games/src/bao.rs (+ mancala4.rs)`.

> Rezumat din `polytabletop list`: Tablă 4 × 8, două rânduri pe jucător, 22 de semințe în rezervă și nyumba cu 6. Namua: captura e obligatorie (pui o sămânță într-o groapă din față, iei groapa opusă și o semeni de la capătul rândului); altfel pui o sămânță și semeni sensul ales. Mtaji: muți dintr-o groapă cu ≥ 2 semințe; captură din prima aterizare obligatorie. Câștigi golind rândul din față al adversarului. Mutări: 3r, 3l (sens), b3l (rândul din spate).

## Scop

Golești rândul din față al adversarului sau îl lași fără mutări.

## Pregătire

Tablă 4 × 8; fiecare jucător deține două rânduri unite într-un inel de 16 gropi. Start: 6 semințe în nyumba (groapa 5 din față), câte 2 în gropile 6 și 7 din față și 22 în rezervă (64 de semințe în total).

## Desfășurarea turei

Namua (cât ai rezervă): dacă poți captura (groapă ocupată din față, față în față cu o groapă ocupată a adversarului), ești obligat: pui o sămânță din rezervă în ea, iei groapa opusă și o semeni de la capătul rândului din față (kichwa): din coloanele 1–2 doar de la stânga, din 7–8 doar de la dreapta, din 3–6 alegi capătul. Fără captură: pui o sămânță într-o groapă ocupată din față, o ridici cu tot și o semeni în sensul ales. Mtaji (fără rezervă): muți dintr-o groapă cu cel puțin 2 semințe, în sensul ales; dacă vreo mutare capturează din prima aterizare, ești obligat să capturezi; altfel muți din față (din spate doar dacă în față nu ai nicio groapă cu 2+).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `3r`, `3l`: coloana 3 din față, semănat spre dreapta (`r`) sau stânga (`l`); la captură sufixul arată de la ce capăt pornește semănatul.
* `b3l`: groapa 3 din rândul din spate (doar în mtaji).

## Reguli speciale și variante alese

* Aterizare: ultima sămânță într-o groapă goală încheie tura; într-o groapă ocupată din față cu opusul ocupat = captură (semințele luate se seamănă de la kichwa, cu sensul păstrat în coloanele 3–6); altfel ridici groapa și continui (takasa).
* Nyumba nu are statut special (fără safari / kutakatia).

## Punctaj

Nu există punctaj; contează golirea rândului din față.

## Sfârșitul jocului

Câștigă cine golește rândul din față al adversarului sau îl lasă fără mutări. După 600 de mutări câștigă cine are mai multe semințe.

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă 4.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar nu are informație ascunsă.

## Diferențe față de regulile tradiționale

* Variantă simplificată: nyumba fără rol special, fără kutakatia / safari; lanțul e limitat la 3000 de semințe.
* Regulile exacte ale capturilor consecutive (sensul de semănat) variază în surse; s-a ales păstrarea sensului în coloanele 3–6 și kichwa forțat în coloanele 1–2 și 7–8.
