# Hotarul (variantă originală) (`hotar`)

Hotarul: motor de resurse și teritorii pe hartă hexagonală, variantă originală pentru doi, cu facțiuni diferite.

## Scop

Ai scorul final mai mare (monede + stele + hexuri controlate + perechi de resurse).

## Pregătire

Harta are 37 de hexagoane (rânduri a–g): câmpie C (grâne), pădure P (lemn), munte M (fier), lac L (impracticabil), două sate natale H și Cetatea Veche X în centru. Fiecare jucător alege o facțiune (`alege luncenii`), are un erou, până la 4 călăreți și până la 6 muncitori.

## Desfășurarea unei ture

Alegi una din cele 4 secțiuni ale tablei tale (nu aceeași ca în tura precedentă), faci acțiunea de SUS (opțională: `sN sari`), apoi cea de JOS (opțională: `gata`): s1 Marș (`s1 mars h:g1>f1`, `s1 bani`; jos `mobilizeaza f1`), s2 Întărire (`s2 putere`, `s2 carti`; jos `imbunatateste mars|productie|comert`), s3 Producție (`s3 produce a1 b2`, `s3 muncitor`; jos `construieste moara|fortareata|monument f1`), s4 Comerț (`s4 comert grane|lemn|fier`, `s4 faima`; jos `inrolare 1..4`).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `alege munteanii` — alegi facțiunea (luncenii, munteanii, padurenii, campenii)
* `s4 comert grane` — secțiunea 4, acțiune de sus: comerț cu grâne
* `inrolare 1` — acțiune de jos: înrolarea de pe locul 1
* `s1 mars h:g1>f1` — marș: eroul h de la g1 la f1
* `lupta P C` — angajezi putere P și carte C (sau `-`)
* `rezolva` — rezolvă lupta

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play hotar --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `alege munteanii` — alegi facțiunea Munteanii (+2 forță în luptă pe munte).
2. `s4 comert grane` — secțiunea Comerț, acțiunea de sus: cumperi grâne.
3. `inrolare 1` — acțiunea de jos: înrolarea 1.

## Reguli speciale și variante alese

Luptă: un erou sau călăreț care intră pe un hex cu unități de luptă inamice declanșează lupta. Atacatorul declară `lupta P C` (putere P ≤ 7 din rezerva ta, carte de valoare C sau `-`), apoi apărătorul face la fel fără să știe, apoi `rezolva`. Forță = unități (erou 2, călăreț 1) + putere + carte (+2 Fortăreață pe hex; +2 muntele pentru munteni); egal câștigă apărătorul. Puterea și cărțile folosite se pierd; cine pierde se retrage acasă; câștigătorul +1 popularitate.

## Punctaj

Scor final: monede + stele × a + hexuri controlate × b + perechi de resurse × c (a, b, c cresc cu popularitatea: 3/2/1, 4/3/2, 5/4/3). Stele: 4 upgrade-uri, 3 călăreți, 3 clădiri, 3 înrolări, putere 12, popularitate 12, prima și a doua victorie în luptă.

## Sfârșitul jocului

La 5 stele sau tura 32.

## Note despre AI și informația ascunsă

Informație ascunsă: cărțile de luptă ale adversarului, valorile angajate în luptă.

## Diferențe față de regulile tradiționale

Joc original; nu se declară abateri.
