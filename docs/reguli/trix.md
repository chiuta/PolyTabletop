# Trix (2 jucători) (`trix`)

Trix (Orientul Mijlociu) pentru doi jucători, cu 32 de cărți și 10 mâini cu contracte diferite.

## Scop

Ai scorul total mai mare după 10 mâini (contractele sunt în mare parte de penalizare).

## Pregătire

Pachet de 32 (7–A), câte 16 fiecăruia (toate cărțile sunt în joc). Primele 5 mâini sunt „împărăția” lui J1, următoarele 5 ale lui J2. `imparte` pornește o mână.

## Desfășurarea unei ture

Proprietarul mâinii alege un contract pe care nu l-a mai folosit, apoi începe. Contractele cu levate: **king** (Popă de cupă: −75), **queens** (−25 pe damă), **diamonds** (−10 pe caro), **slaps** (−15 pe levată); se răspunde la culoare, fără atu. **Trex**: aranjament care începe cu un Valet din orice culoare; apoi cărți vecine în jos (10, 9, 8, 7) sau în sus (Q, K, A); cine nu poate pune spune `pas`; primul rămas fără cărți ia +200.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `king / queens / diamonds / slaps / trex` — alegi contractul
* `Jh` — joci/pui o carte
* `pas` — doar la Trex, când nu poți pune
* `imparte` — începe mâna următoare

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play trix --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `diamonds` — alegi contractul „diamonds” (−10 pentru fiecare caro luat).
2. `7s` — joci 7 de pică.
3. `Ad` — joci Asul de caro.

## Reguli speciale și variante alese

Mâna se încheie când au căzut toate cărțile penalizate. Un contract nu poate fi folosit de două ori de același proprietar.

## Punctaj

Penalizările de mai sus; Trex +200 primului care termină.

## Sfârșitul jocului

Câștigă cine are scorul total mai mare după 10 mâini.

## Note despre AI și informația ascunsă

Informația ascunsă: AI-ul citește doar propria mână.

## Diferențe față de regulile tradiționale

Dublările (kawa) nu sunt incluse; doi jucători.
