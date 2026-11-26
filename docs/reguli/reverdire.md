# Reverdire (variantă originală) (`reverdire`)

Reverdire: motor de resurse și proiecte de cărți pentru doi: două companii redau viața unui bazin sărat secat (variantă originală).

## Scop

Ai scorul mai mare (RR + puncte de carte + arbori) după ce cei trei parametri globali ajung la maxim.

## Pregătire

Trei parametri globali: Umiditate (0–10), Aer (0–8), Lacuri (0–7, câte o plăcuță-lac pe harta 5×5). Fiecare pas ridică Ratingul de Reverdire (RR). Șase resurse: Lei (L), Cărămizi (C), Cristale (K), Semințe (S), Curent (E), Căldură (H), fiecare cu producție proprie.

## Desfășurarea unei ture

O generație: (1) `generatie` (mutare forțată de început); Cercetare — primești 4 cărți și cumperi oricâte cu 3 Lei fiecare (`compra 2`, `gata`); (2) acțiuni pe rând, una pe tură, până pasează ambii; (3) producție: Lei += RR + producția de Lei, Curentul vechi se face Căldură, apoi se adaugă producțiile.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `generatie` — începe o generație nouă (forțat)
* `compra 4` — cumperi cartea 4 din ofertă (3 Lei)
* `gata` — termini cercetarea
* `joc 3` — joci proiectul 3 din mână
* `vinde 3` — vinzi cartea 3 (+1 Leu)
* `pompa` — 10 L → +1 producție de Curent
* `irigare` — 16 L → un lac
* `plantare` — 20 L → un arbore
* `asezare` — 22 L → un oraș, +1 producție de Lei
* `conv sem / conv caldura` — conversii (6 Semințe → arbore; 6 Căldură → Umiditate +1)
* `pas` — pasezi
* `pune c3` — alegi locul unei plăcuțe

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play reverdire --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `generatie` — începe prima generație (mutare forțată).
2. `compra 4` — cumperi cartea 4 din oferta ta de cercetare.
3. `compra 3` — cumperi cartea 3.

## Reguli speciale și variante alese

Cărămizile valorează 2 Lei la proiectele cu etichetă B, Cristalele 3 Lei la cele cu etichetă S; unele proiecte cer un parametru minim. Plăcuțe: lac pe una din cele 7 celule-lac; arbore pe pământ, lipit de o plăcuță a ta dacă se poate (Aer +1); oraș pe pământ, nelipit de alt oraș. O plăcuță de pământ lipită de lacuri dă +2 Lei pentru fiecare lac vecin.

## Punctaj

RR + punctele cărților + 1 pe arbore + 1 pe arborele vecin fiecărui oraș al tău. Egalitate: mai mulți Lei.

## Sfârșitul jocului

După generația în care toți cei trei parametri sunt la maxim (sau după a 16-a).

## Note despre AI și informația ascunsă

Informație ascunsă: pachetul, mâna și oferta adversarului; AI-ul (alpha-beta adâncime 2) nu le citește. Cărțile sunt generate.

## Diferențe față de regulile tradiționale

Joc original; nu se declară alte abateri.
