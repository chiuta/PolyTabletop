# Împletitura (echilibru pe covor cu cercuri colorate) (`impletitura`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play impletitura --seed 1`. Sursa implementării: `polytabletop-games/src/impletitura.rs`.

> Variantă originală, cu reguli proprii (idee generică de joc de echilibru pe covor cu cercuri colorate și rotiță); titlul în engleză: „Tangle Mat”. Nu folosește nume, texte sau elemente grafice ale vreunui produs comercial.

## Scop

Să rămâi ultimul „în picioare”: cine nu mai are niciun cerc valid pentru membrul și culoarea date de rotiță cade și pierde.

## Pregătire

Covorul are 4 rânduri (a–d) × 6 coloane (1–6) de cercuri în 4 culori (Roșu `R`, Albastru `A`, Verde `V`, Galben `G`), câte 6 din fiecare; așezarea culorilor depinde de `--seed`. Fiecare jucător are 4 membre: mâna stângă (`ms`), mâna dreaptă (`md`), piciorul stâng (`ps`) și piciorul drept (`pd`). Toate încep în afara covorului. J1 începe.

## Desfășurarea turei

1. **Rotița.** Tura începe cu mutarea forțată `gira`. Rotița dă un membru și o culoare (de exemplu „piciorul stâng pe Verde”). Rotița este extrasă din generatorul determinist al jocului, deci același `--seed` dă aceeași partidă.
2. **Mutarea.** Muți acel membru pe un cerc de culoarea dată, scriind numele cercului (`b3` = rândul b, coloana 3). Cercul trebuie să fie:
   * **liber**: un cerc nu poate fi ocupat de două membre (regula de „încurcare” — nici de ale aceluiași jucător, nici de ale adversarului); membrele adversarului îți blochează cercurile;
   * **în rază**: la distanță Chebyshev (maximul dintre diferența de rânduri și diferența de coloane) cel mult 3 față de celelalte membre ale tale aflate deja pe covor.
   Poți rămâne pe cercul pe care stă deja membrul, dacă are culoarea dată (scrii numele lui). Cercul vechi se eliberează.
3. **Cădere.** Dacă nu există niciun cerc valid, singura mutare este `cad`: cazi, ești eliminat și adversarul câștigă. Mutări ilegale nu există: lista `?` conține doar cercurile valide.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `gira` — învârți rotița (singura mutare la începutul turei).
* `b3` — pui membrul rotit pe cercul b3.
* `cad` — nu ai cerc valid: cazi.

## Reguli speciale și variante alese

* Doar doi jucători (motorul jocurilor este pentru doi); „ultimul rămas” înseamnă adversarul tău.
* Rotița este o mutare separată (`gira`) ca AI-ul să nu poată ști rezultatul înainte să-l facă public.
* Regula de siguranță: după 300 de mutări de membre fără nicio cădere, jocul este REMIZĂ (rar în practică).
* Fără informație ascunsă; singurul hazard este rotița.

## Punctaj

Fără punctaj: câștigă cine nu cade.

## Sfârșitul jocului

Când un jucător cade (`cad`) — celălalt câștigă — sau după 300 de mutări de membre (remiză).

## Note despre AI

Informație completă. AI-ul alege la adâncime mică (alpha-beta 1 strat; Hard/Expert 2 straturi, al doilea fiind doar `gira`) și nu citește rotița următoare, deci nu trișează. Evaluarea măsoară „flexibilitatea”: pentru fiecare pereche membru × culoare, câte cercuri valide rămân (cu penalizare mare dacă nu mai rămâne niciunul).

* **Easy** — aproape aleator (evaluare zgomot determinist; ocazional alege a 2-a/3-a mutare).
* **Normal** — își maximizează propria flexibilitate.
* **Hard / Expert** — maximizează diferența dintre flexibilitatea lui și a ta: ocupă cercurile care îți lasă fără variante.

Stil AI în cod: alpha-beta cu adâncime fixă 1 (nivelul global îl crește la 2 pentru Hard/Expert).

Modul „la masă” (`--you both`): nu există informație ascunsă; jocul este în lista NO_HIDDEN din `docs/HOTSEAT.md`.

## Diferențe față de regulile tradiționale

Jocul tradițional de acest tip se joacă fizic cu 2–4 persoane pe un covor mare; aici: 2 jucători, covor 4×6 cu raza maximă 3 între membre, rotița ca mutare și cădere ca lipsă de cerc valid (nu pierderea echilibrului fizic).
