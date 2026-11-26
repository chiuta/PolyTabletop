# Cribbage (`cribbage`)

Cribbage cu 6 cărți pentru doi jucători, cu tablă de colac de 121 de găuri desenată în text.

## Scop

Ajungi primul la 121 de puncte prin pegging (jocul cărților) și numărarea mâinilor și a cribului.

## Pregătire

Pachet de 52. Fiecare primește 6 cărți și dă 2 în „crib” (al dealerului). Non-dealerul taie cartea de start; un Valet de start dă dealerului 2 puncte („his heels”).

## Desfășurarea unei ture

Faza 1: fiecare dă 2 cărți în crib. Faza 2: se taie startul. Faza 3 (pegging): se joacă pe rând cărți, suma curentă nu depășește 31; se punctează 15 (2), 31 (2), perechi (2/6/12), serii de 3+, „go”/ultima carte (1). Faza 4: numărarea (`numara`) în ordinea non-dealer, dealer, crib; apoi `imparte` pentru mâna următoare.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `d 5h 6s` — dai două cărți în crib
* `taie` — tai cartea de start
* `5h` — joci o carte în pegging
* `go` — nu poți juca, spui go
* `numara` — treci la numărare
* `imparte` — împarte următoarea mână

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play cribbage --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `d 8c 6c` — dai 8 și 6 de treflă în crib.
2. `taie` — tai cartea de start (suma de pornire).
3. `Qs` — joci Dama de pică în pegging.

## Reguli speciale și variante alese

Numărarea: 15-uri (2 fiecare), perechi, serii, culoare (4 în mână, 5 cu start; cribul doar 5), valetul de aceeași culoare cu startul (1). Primul la 121 câștigă pe loc, chiar în mijlocul numărării.

## Punctaj

Tabla afișată: `●` țăruș din față, `○` din spate; 121 de găuri. Punctele se adaugă imediat în pegging și la numărare.

## Sfârșitul jocului

Primul care atinge 121 câștigă imediat.

## Note despre AI și informația ascunsă

Informație ascunsă: `render` arată mâna Jucătorului 1; cribul și mâna adversarului apar doar când se numără. AI-ul (alpha-beta adâncime 1) citește doar propria mână și informațiile publice.

## Diferențe față de regulile tradiționale

Tabla de colac este desenată în text; cele 6 cărți (2 în crib) sunt varianta standard pentru doi jucători.
