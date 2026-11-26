# Tong-its (`tongits`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tongits --seed 1`. Sursa implementării: `polytabletop-games/src/tongits.rs`.

> Rezumat din `polytabletop list`: 52 de cărți, 12 fiecare (dealerul 13 și începe fără să tragă). Tura: trage din stoc sau ia aruncata dacă o folosești imediat (ia 5d 5h 5c / ia sapaw 2), apoi pune seturi sau scări (pune 3h 4h 5h), adaugă la orice combinație (sapaw 6h 2) și aruncă (arunca Kd). Mână goală = Tongits, 3 p.; cheama (cu o combinație pusă) → adversarul renunt (1 p.) sau lupta: mai puține puncte în mână câștigă (egalitate: cel care a luptat; dacă pierde cel care a chemat, adversarul ia 2 p.); stoc epuizat: mai puține puncte. Primul la 6 p.

## Reguli implementate, variantă aleasă și omisiuni

Se alege varianta cea mai răspândită; ce lipsește este menționat explicit mai jos (rânduri cu „Omis”).

Tong-its / Tongits (Filipine) — rummy cu apel „draw”, 2 jucători.

Reguli folosite (varianta cea mai răspândită, adaptată pentru doi):
* 52 de cărți. Dealerul primește 13 cărți, celălalt 12; dealerul începe (poate pune combinații,
apoi aruncă o carte, fără să tragă). Apoi fiecare tură: tragi din stoc (`trage`) sau iei cartea
de sus din aruncate dacă o folosești imediat într-o combinație (`ia 5d 5h 5c`) sau o
adaugi la una existentă (`ia sapaw N`); apoi poți pune combinații (`pune 3h 4h 5h`), poți
adăuga cărți la orice combinație de pe masă, și ale adversarului (`sapaw 6h N`), și
arunci o carte (`arunca Kd`).
* Combinații: set de 3–4 cărți de același rang (culori diferite) sau scară de cel puțin 3 cărți
consecutive de aceeași culoare (As doar jos: A-2-3, nu Q-K-A). Valori: A 1, 2–10 cât scrie,
J/Q/K 10.
* Câștigi mâna: (a) rămâi fără cărți („Tongits”): 3 puncte; (b) apelezi `cheama` (draw) la
începutul turei, dacă ai cel puțin o combinație pusă: adversarul spune `renunt` (tu iei 1
punct) sau `lupta`: se compară cărțile rămase în mână (suma valorilor) — cine are mai puțin
câștigă 1 punct; la egalitate câștigă cel care a luptat; dacă cel care a chemat pierde,
adversarul primește 2 puncte; (c) stocul se golește: după aruncarea celui care a tras
ultima carte, cine are mai puține puncte în mână câștigă 1 (egalitate = fără punct).
* Meciul: primul la 6 puncte. Dealerul alternează.
* Omise: combinații ascunse („secret melds”), regula „burn”, 3 jucători, miza de jetoane;
punctajul e o adaptare simplă a plăților.
* Informație ascunsă: vezi doar mâna ta; combinațiile și aruncatele sunt publice. AI-ul
minimizează cărțile rămase în mână și evită să arunce ce adversarul poate folosi.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Evenimentele de șansă (zaruri, împărțire, tragere) sunt mutări forțate.
