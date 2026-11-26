# Kings (Ring of Fire) (`kings`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play kings --seed 1`. Sursa implementării: `polytabletop-games/src/kings.rs`.

> Rezumat din `polytabletop list`: Kings fără alcool: tragi pe rând cărți cu reguli pe rang; înghițiturile sunt jetoane de penalizare; al patrulea rege te face să iei tot paharul. Mutări: trag, scut, atac.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Regii (Kings / Ring of Fire, SUA), 2 jucători — versiune fără alcool, cu jetoane de penalizare.


Jocul de petrecere cu pachet de 52 de cărți în care fiecare rang are o regulă. Aici „înghițiturile” devin

**jetoane de penalizare**; câștigă cine are mai puține la sfârșit. Se trage pe rând o carte (`trag`):

* **A cascadă**: ambii +1. **2 tu**: adversarul +2. **3 eu**: tu +2. **4 podea**: unul dintre voi (la sorți) +1.

* **5 echipă**: adversarul +1. **6 echipă**: tu +1. **8 partener**: unul dintre voi (la sorți) +1.

* **9 rimă**: adversarul +1. **10 categorie**: tu +1. **Q întrebări**: unul dintre voi (la sorți) +2.

* **7 cer** și **J regulă**: alegi (`scut` = următoarele 2 jetoane care ți se dau nu se pun; `atac` =

  adversarul +1; la J, atacul dă +2).

* **K rege**: primii trei regi adaugă 2 jetoane în pahar și tu +1; al patrulea rege te face să iei **tot

  paharul** și încheie jocul. Dacă pachetul se termină, jocul se oprește.

* Cartea se trage cu generatorul jocului la mutarea `trag`, deci AI-ul nu o poate prevedea.


Mutări: `trag`, `scut`, `atac`.
