# Table chinezească (Chinese Checkers) (`chinese_checkers`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play chinese_checkers --seed 1`. Sursa implementării: `polytabletop-games/src/chinese_checkers.rs`.

> Rezumat din `polytabletop list`: Tablă stelată cu 121 de găuri, 10 piese pe jucător: pas pe o gaură vecină sau salturi înlănțuite peste piese; câștigi umplând triunghiul opus. După 300 de mutări se compară progresul. Mutări: d4-e5 (fără traseul sărit).

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Table chinezească (Sternhalma / Chinese Checkers, Germania), 2 jucători, din regulile publice.


* Tabla în formă de stea cu 121 de găuri (17 rânduri: 1-2-3-4-13-12-11-10-9-10-11-12-13-4-3-2-1).

  Fiecare jucător are 10 piese în triunghiul său (J1 sus, J2 jos) și trebuie să ocupe triunghiul

  opus.

* O mutare: un pas pe o gaură vecină liberă (6 direcții) SAU o serie de salturi peste piese (ale

  oricui) într-o gaură liberă imediat dincolo, în aceeași direcție, cu salturi înlănțuite.

  Mutarea se scrie ca `de-la`, fără traseul (două trasee spre aceeași gaură = aceeași mutare).

* Câștigă primul care umple triunghiul opus (toate cele 10 găuri ocupate, cel puțin una a ta; un

  adversar care își lasă piese acolo nu te poate bloca la infinit pentru că se aplică și limita).

* Limita noastră de joc: după 300 de mutări se compară progresul (suma rândurilor parcurse);

  egalitate = remiză.


Găurile se numesc cu litera rândului (a–q, de sus în jos) și numărul din rând (de la stânga):

`a1`, `e13`. Mutări: `d4-e5`.
