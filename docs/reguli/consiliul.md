# Sfatul Negustorilor (alegeri și legi, roluri ascunse) (`consiliul`)

Sfatul Negustorilor: joc de alegeri și legi cu roluri politice ascunse; 5 locuri (tu + 4 roboți).

## Scop

Negustorii: 5 legi P sau executarea Cămătarului. Conspiratorii: 6 legi M sau Cămătarul cancelar după 3 M.

## Pregătire

Trei Negustori (cinstiți), un Cămătar (rol-cheie) și un Complice. Complicele și Cămătarul se cunosc; negustorii nu știu nimic. Pachetul are 6 legi P (Prosperitate) și 11 legi M (Monopol).

## Desfășurarea unei ture

Președintele nominalizează un Cancelar (`cancelar 3`, nu pe cel anterior), toți votează `da`/`nu`. Dacă trece: Președintele trage 3 legi, aruncă una (`arunca M`), Cancelarul promulgă una din cele 2 rămase (`promulga P`). La a 3-a lege M Președintele vede următoarele 3 legi; la a 4-a și a 5-a execută un jucător (`executa 4`). Trei alegeri respinse la rând: se promulgă legea din vârful pachetului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `cancelar 3` — nominalizezi cancelar
* `da / nu` — votezi guvernul
* `arunca M` — arunci o lege (M sau P)
* `promulga P` — promulgi o lege
* `executa 4` — execuți un jucător
* `polytabletop` — roboții acționează (forțat)

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play consiliul --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `cancelar 3` — nominalizezi locul 3 ca Cancelar.
2. `nu` — votezi `nu` pentru un guvern propus de locul 2 (o respingere).
3. `arunca M` — arunci o lege M din cele trei.

## Reguli speciale și variante alese

Nu se poate nominaliza cancelarul anterior.

## Punctaj

Afișajul arată P x/5, M x/6 și respingerile x/3.

## Sfârșitul jocului

Negustorii câștigă cu 5 P sau executând Cămătarul; conspiratorii cu 6 M sau dacă Cămătarul devine Cancelar după 3 M.

## Note despre AI și informația ascunsă

Roboții folosesc doar informația publică plus propriul rol; rolurile tale (Cămătar) sunt afișate doar ție.

## Diferențe față de regulile tradiționale

Simplificări declarate: fără vetou, fără investigarea loialității, 5 jucători fix.
