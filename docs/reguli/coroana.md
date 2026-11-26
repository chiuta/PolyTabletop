# Drumul spre Coroană (aventură pe inele, om contra roboți) (`coroana`)

Drumul spre Coroană: aventură pe inele concentrice, om contra roboți (variantă originală).

## Scop

Fii ultimul în joc: ceilalți doi aventurieri trebuie scoși din joc prin Porunca Coroanei sau în luptă.

## Pregătire

Trei aventurieri (tu și doi roboți) alegi un personaj (`alege haiducul`, `vrajitoarea`, `negustorul`, `padurarul`, `pelerinul`, `calauza`). Pornesc din Câmpie (inelul exterior, 16 câmpuri), trec prin Podul Gri în Țara Cenușie (12 câmpuri), își câștigă Pecetea în Turnul Peceții, urcă Scara Neagră în Inelul de Jar (8 câmpuri) și intră pe Ușa Coroanei în camera din centru.

## Desfășurarea unei ture

Arunci zarul de mișcare (`zar`, d6, +1 cu Cizmele Iuțe), alegi sensul (`dreapta`/`stanga`) și rezolvi câmpul: cărți de aventură (monștri, obiecte, întâmplări, descântece), han, capelă, târg (`cumpar sabie`, `plec`), mlaștină, pădure, altar, tâlhari, lac de jar. Pe Podul Gri: `urc plata`, `urc cheie` sau `urc lupta`. În luptă: `lupt`, `lupt fulger`, `fug`, `ceata`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `alege padurarul` — alegi personajul
* `zar` — arunci zarul de mișcare
* `dreapta / stanga` — alegi sensul de mers
* `lupt / lupt fulger / fug / ceata` — în luptă
* `bea / leac` — la han / cu un obiect
* `urc / urc plata / urc cheie / urc lupta` — treci Podul sau Scara
* `intru` — intri pe Ușa Coroanei
* `mantin` — menții poziția în cameră
* `polytabletop` — roboții acționează (forțat)

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play coroana --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `alege padurarul` — alegi Pădurarul (fuga reușește mereu; Pădurea nu-l rănește).
2. `zar` — arunci zarul (iese 3).
3. `stanga` — mergi 3 câmpuri spre stânga.

## Reguli speciale și variante alese

Lupta: Forța ta + d6 trebuie să DEPĂȘEASCĂ forța monstrului (egal = nimic). Cine pierde toată Viața în afara camerei „cade”: pierde Aurul, un obiect și Pecetea, se trezește la Han cu Viață pe jumătate și pierde o tură. Cine stă în cameră își folosește Porunca la începutul fiecărei ture: ceilalți pierd o Viață; la 0 sunt scoși din joc. Dacă doi stau în cameră, se bat între ei.

## Punctaj

Nu există scor; contează ultimul rămas.

## Sfârșitul jocului

Câștigi când ceilalți doi sunt scoși din joc; limită de 150 de runde (apoi câștigă cine e mai înaintat).

## Note despre AI și informația ascunsă

Roboții joacă după euristici pe starea publică.

## Diferențe față de regulile tradiționale

Simplificări declarate: fără aliați, fără alegerea obiectului de aruncat (se aruncă cel mai slab), limită de 150 de runde.
