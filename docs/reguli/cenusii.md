# Noapte Cenușie (cooperativ de supraviețuire pe grilă) (`cenusii`)

Noapte Cenușie: joc cooperativ de supraviețuire pe grilă 7×6 cu doi supraviețuitori (Ilinca = J1, Radu = J2, AI).

## Scop

Strângeți 3 baterii (B) și ajungeți amândoi la ieșire (E), ținând piept „umbrelor”.

## Pregătire

42 de zone (coloanele a–g, rândurile 1–6), două zone de apariție (S), ziduri (#). Arme: Ciomag, Topor, Praștie, Arbaletă, Pistol; un obiect găsit se pune în mâna liberă (altfel se pierde).

## Desfășurarea unei ture

3 acțiuni pe tură: `mers n|s|e|v` (1 acțiune; 2 dacă pleci dintr-o zonă cu umbre), `atac 1 c3` (arma din mâna 1 asupra zonei c3; arme de corp la corp doar în zona ta, cele de la distanță în linie dreaptă, fără ziduri), `cauta` (o dată pe tură, într-o zonă fără umbre), `ia` (o baterie din zonă), `pansez` (vindeci o rană), `gata`. După fiecare tură: `umbre` (cele din zona unui supraviețuitor lovesc, celelalte merg spre cel mai apropiat) și `aparitie` (în zonele S).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `mers n` — mergi spre nord (s, e, v analog)
* `atac 1 c3` — arma din mâna 1 asupra zonei c3
* `cauta` — extragi echipament
* `ia` — iei o baterie din zonă
* `pansez` — vindeci o rană
* `gata` — termini acțiunile
* `zaruri / extrage / umbre / aparitie / sfârșit` — mutări forțate

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play cenusii --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `mers v` — mergi spre vest.
2. `mers n` — mergi spre nord (în c5 este o baterie).
3. `ia` — iei bateria (1/3).

## Reguli speciale și variante alese

Umbre: rătăcit (viață 1, 1 pas), alergător (viață 1, activează de două ori), colos (viață 2, lovește cu 2; o armă cu forță < 2 nu-l poate omorî). Zarurile și extragerea echipamentului sunt mutări forțate separate (`zaruri`, `extrage`).

## Punctaj

Afișajul arată bateriile (x/3), umbrele de pe hartă, rănile (x/4) și pansamentele.

## Sfârșitul jocului

3 baterii și ambii vii la ieșire = victorie (J1 „câștigă”). Moartea unuia (4 răni), 24 de umbre pe hartă sau 40 de ture = înfrângere = REMIZĂ.

## Note despre AI și informația ascunsă

Cooperativ; Radu (J2) este AI. Zarurile sunt mutări forțate separate ca AI-ul să nu vadă rezultatul înainte să aleagă.

## Diferențe față de regulile tradiționale

Simplificări declarate: fără uși/clădiri, fără experiență și abilități, fără foc prietenesc, alegerea țintelor de către armă e fixă (cele mai slabe umbre primele).
