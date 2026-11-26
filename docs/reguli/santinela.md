# Străjerii Farului (misiuni cu echipe, roluri ascunse) (`santinela`)

Străjerii Farului: joc de echipe și deducție socială, 5 locuri (tu + 4 roboți). Trei paznici (unul e Ochiul) și doi sabotori (unul e Vulpea).

## Scop

Paznicii: 3 misiuni reușite. Sabotorii: 3 misiuni ratate sau 5 respingeri la rând.

## Pregătire

Cinci locuri; rolul tău e afișat (aici Vulpea). Cinci misiuni cu echipe de 2, 3, 2, 3, 3. Ochiul știe cine sunt sabotorii; sabotorii se cunosc.

## Desfășurarea unei ture

Conducătorul propune o echipă (`echipa 1 3`), toți votează `da`/`nu`; la 5 respingeri la rând câștigă sabotorii. Pe o misiune aprobată fiecare membru alege în secret `succes` sau `sabotaj` (doar sabotorii pot sabota; o singură sabotare ratează misiunea). `polytabletop` lasă roboții să acționeze.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `echipa 1 3` — propui echipa formată din locurile 1 și 3
* `da / nu` — votezi echipa propusă
* `succes / sabotaj` — alegi rezultatul misiunii
* `tinta 4` — Vulpea ghicește Ochiul
* `polytabletop` — roboții acționează (forțat)

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play santinela --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `echipa 4 5` — propui echipa de locurile 4 și 5.
2. `nu` — votezi `nu` pentru echipa propusă de locul 2.
3. `nu` — votezi `nu` pentru echipa propusă de locul 3.

## Reguli speciale și variante alese

Dacă paznicii au 3 reușite, Vulpea ghicește Ochiul: dacă nimerește, sabotorii câștigă totuși.

## Punctaj

Afișajul arată misiunile reușite/ratate (paznici : sabotori) și respingerile (x/5).

## Sfârșitul jocului

Primul care ajunge la 3 misiuni reușite (paznici) sau 3 ratate (sabotori) câștigă (cu excepția ghicirii Ochiului).

## Note despre AI și informația ascunsă

Roboții folosesc doar informația publică (cine a propus/votat, ce misiuni au eșuat) plus propriul rol (sabotorii se cunosc, Ochiul îi cunoaște pe sabotori).

## Diferențe față de regulile tradiționale

Variantă originală de „misiuni cu roluri ascunse”; 5 jucători fix.
