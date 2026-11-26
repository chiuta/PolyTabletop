# Tărâmuri (transformarea terenului și piste de ritual) (`taramuri`)

Tărâmuri: joc de transformare a terenului și de ritual pe patru piste, pe hartă 7×6, 2 jucători, informație completă (seed-ul stabilește harta și bonusurile).

## Scop

Ai mai multe puncte după 6 runde.

## Pregătire

7 tipuri de teren pe o roată (0–6). J1 e de tip 0, J2 de tip 3. Un teren la distanță `d` pe roată de tipul tău costă `d` lopeți; o lopată costă 2 muncitori (1 dacă ai Pământ ≥ 5). Început: pe rând J1, J2, J2, J1 pui câte o locuință pe un teren natal liber (`inicial c3`). Resurse: 3 muncitori, 5 monede, 1 preot.

## Desfășurarea unei ture

Pe rând faci o singură acțiune sau `pas` (după pas nu mai joci în runda respectivă; primul care dă pas începe runda următoare): `colonie c3`, `oras c3`, `templu c3`, `preot foc|apa|pamant|aer`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `inicial g4` — pui locuința inițială pe g4
* `colonie f4` — transformi terenul f4 și pui o locuință (celulă fără clădire, vecină cu o clădire a ta)
* `oras e3` — locuință → târg
* `templu e3` — târg → templu
* `preot aer` — un preot avansează 2 trepte pe pista Aer (la 5 privilegiu)
* `pas` — nu mai joci în runda aceasta

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play taramuri --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `inicial g4` — locuința inițială pe g4.
2. `inicial e3` — a doua locuință inițială pe e3.
3. `preot aer` — preotul avansează 2 trepte pe pista Aer.

## Reguli speciale și variante alese

Locuință: 1 muncitor + 2 monede, cel mult 8. Târg: 2 muncitori + 3 monede, cel mult 4. Templu: 4 muncitori + 6 monede, cel mult 2, +1 preot. Pistele (0–10): la 5 privilegiu permanent: Foc +2 muncitori pe rundă, Apă +2 monede pe rundă, Pământ lopată mai ieftină, Aer rază 2. Venit la sfârșitul rundei: muncitori = 1 + locuințe (+2 Foc ≥ 5), monede = 2 + 2 × târguri + 4 × temple (+2 Apă ≥ 5). Bonus de rundă (ordinea depinde de seed): +2 pe locuință nouă, +3 pe târg, +5 pe templu, +2 pe lopată, +1 pe treaptă de cult, +1 pe orice clădire.

## Punctaj

La final: pe fiecare pistă primul +8, al doilea +4 (egal: 6 și 6); cel mai mare grup de clădiri vecine +10 (egal: 5 și 5); 1 punct la 3 monede.

## Sfârșitul jocului

După runda 6.

## Note despre AI și informația ascunsă

Informație completă; AI-ul (alpha-beta adâncime 6) vede toată starea.

## Diferențe față de regulile tradiționale

Simplificări declarate: hartă pătrată 7×6 cu vecinătate pe 4 direcții (nu hexagoane), fără puterea/ciclul de mană, fără facțiuni diferite (doar teren natal diferit), fără fortărețe, fără schimb de puncte pentru surplus.
