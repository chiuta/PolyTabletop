# Cărți colecționabile: monștri în zone și capcane (Hotarele Ceții) (`tcg_zone`)

Cărți colecționabile cu monștri în zone și capcane („Seria Hotarelor Ceții”), 2 jucători, set original de 44 de cărți.

## Scop

Reduci la zero cei 8000 PV ai adversarului sau îl lași fără pachet.

## Pregătire

Pachet pregătit de 30 de cărți (J1 „Hotarul”, J2 „Mlaștina”), 5 cărți în mână, 8000 PV. Cinci zone de monștri și cinci zone de capcane.

## Desfășurarea unei ture

`pesca` (forțat; pachet gol = pierdere), faza principală, `fin`. O invocare de monstru pe tură: nivel 1–4 fără tribut, 5–6 cu un tribut, 7+ cu două: `invoca 2` (atac) / `seteaza 2` (întors, în apărare); cu tribut `invoca 2 tribut 1,3`. `intoarce I` și `schimba I` pentru monștri. Vrăji: `joaca 3 [țintă]` (țintă `e2`, `m1`, `g2`). Capcane: `seteaza N`. Luptă: `ataca 1 e2` sau `ataca 1 direct`. După fiecare atac apărătorul primește mutarea forțată `continua` sau `activa T`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `pesca` — tragi o carte (forțat)
* `invoca 5` — invoci monstrul 5 în atac
* `seteaza 2` — pui cartea 2 întoarsă (monstru în apărare sau capcană)
* `invoca 2 tribut 1,3` — invocare cu tribut
* `intoarce I / schimba I` — răsucești / schimbi poziția
* `joaca N [e2|m1|g1]` — joci o vrajă
* `ataca 1 e2 / ataca 1 direct` — atacul
* `continua / activa T` — fereastra de capcane
* `fin` — încheie tura

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play tcg_zone --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `pesca` — tragi o carte (mutare forțată).
2. `invoca 5` — invoci Gnom Fierar (nivel 3) în poziție de atac.
3. `seteaza 2` — pui Gaură Adâncă (capcană) întoarsă.

## Reguli speciale și variante alese

ATK vs ATK: cel slab e distrus, diferența îl costă PV pe stăpân; ATK vs APĂRARE: dacă atacă mai puternic, apărătorul e distrus (fără daune PV, dar „Pătrunde” trece diferența), altfel atacatorul pierde diferența. „Vânător”: când distruge un monstru, adversarul pierde 500 PV. J1 nu atacă în tura 1. Fereastra de capcane se deschide mereu, ca AI-ul să nu deducă ce ascunde adversarul.

## Punctaj

Puncte de viață (PV), 8000 la început.

## Sfârșitul jocului

PV la zero sau pachet gol.

## Note despre AI și informația ascunsă

`render` arată doar mâna J1 și propriile cărți întoarse; cele ale adversarului apar ca „întoarse”. AI-ul (alpha-beta adâncime 1) citește doar ce vede; lupta contra unui monstru întors nu e simulată.

## Diferențe față de regulile tradiționale

Set original de cărți, reguli „scrise de noi” din mecanici generice.
