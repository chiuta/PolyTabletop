# Celebrity (Fishbowl) (`celebrity`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play celebrity --seed 1`. Sursa implementării: `polytabletop-games/src/celebrity.rs`.

> Rezumat din `polytabletop list`: Celebrity: vas cu 12 nume, trei runde (descrii, mimezi, un singur cuvânt); coechipierul simulat ghicește; câștigă cine ghicește mai multe. Numele de sus se vede doar de cel care îl dă. Mutări: dau, sar, ghicesc.

`HOTSEAT_SAFE`: da.

## Reguli implementate (varianta aleasă)

Celebrity / Fishbowl (joc de petrecere), 2 echipe — fiecare echipă = un jucător + un coechipier simulat.


* Vasul conține 12 nume de personalități (istorice sau fictive) alese la sorți din 20. Se joacă **trei runde** cu

  același vas: (1) **descrii** în cuvinte, (2) **mimezi**, (3) spui **un singur cuvânt**. Rundele au câte un

  indiciu pregătit pentru fiecare nume (se vede doar la mutarea ta).

* La rândul tău (3 încercări) vezi numele de sus: `dau` = dai indiciul rundei, iar coechipierul ghicește (șansă

  80% în runda 1, 65% în a doua, 50% în a treia, minus 10 puncte procentuale pentru fiecare nivel de dificultate

  peste 1); `sar` = pui numele la fundul vasului (și pierzi încercarea). Numele ghicite se scot din vas; ghicitul

  dă 1 punct. Runda se termină când vasul e gol; câștigă la final cine are mai multe puncte.

* Coechipierul ghicește în mutarea forțată `ghicesc`, deci AI-ul nu cunoaște rezultatul înainte să aleagă.


Mutări: `dau`, `sar`, `ghicesc`. Informație ascunsă: numele de sus se arată doar celui care îl dă (`set_viewer`).
