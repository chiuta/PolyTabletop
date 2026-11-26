# Patrula (skirmish cu echipă mică) (`patrula`)

Patrula: skirmish cu echipă mică de miniaturi individuale (Lider, 2 Pușcași, Greu, Asalt) pe hartă 10×10.

## Scop

Ai mai multe puncte de obiectiv după 6 runde (apoi mai multe răni rămase) sau elimini patrula adversă.

## Pregătire

Hartă 10×10 cu ziduri (`##`, blochează mersul și vederea), acoperiri (`::`, +1 la salvare) și 4 obiective (`()`). Fiecare miniatură are răni proprii, armă de foc și armă de luptă.

## Desfășurarea unei ture

Runda: fiecare miniatură se activează o dată; părțile alternează câte o activare (J1 începe rundele impare). La activare alegi un ORDIN: Avans, Țintă, Asalt, Retragere sau Gardă. Zarurile se aruncă la mutarea forțată `arunca`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `B>d4` — Avans: mută ≤ mv
* `B>d4 xK` — Avans și trage după (−1 la lovire; Greul nu poate trage după ce a mutat)
* `B xK` — Țintă: stai pe loc; dacă K e lipit = luptă, altfel tragi cu +1 la lovire
* `B!d4 xK` — Asalt: mută ≤ mv+2 lângă K și luptă (+1 zar)
* `B<d4` — Retragere: doar dacă ești lipit
* `B garda` — +1 la salvare până la următoarea activare
* `arunca` — aruncă zarurile

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play patrula --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `D!d8 xX` — Greul (D) asaltează Pușcașul X la d8.
2. `arunca` — `arunca` rezolvă zarurile.
3. `A!b8 xZ` — Liderul (A) asaltează Asaltul Z la b8.

## Reguli speciale și variante alese

Foc: linie de vizibilitate liberă și distanță ≤ rază; lovire la d6 ≥ ținta armei (+1 dacă ai mutat sau ești rănit, −1 la Țintă); fiecare lovitură e salvată de un d6 ≥ salvarea apărătorului (−1 în acoperire, −1 în gardă). Rănit = la sau sub jumătate din răni: mv −1. Intrarea într-un pătrat vecin cu un inamic oprește mutarea. Luptă: atacatorul lovește, supraviețuitorul răspunde.

## Punctaj

La sfârșitul rundei, cine are mai multe miniaturi la ≤ 1 pătrat de un obiectiv ia 1 punct.

## Sfârșitul jocului

După 6 runde câștigă cine are mai multe puncte, apoi mai multe răni rămase; eliminarea patrulei adverse câștigă imediat.

## Note despre AI și informația ascunsă

Informație completă (zarurile cu sămânță care se schimbă la `clone`, deci AI-ul nu poate anticipa); AI = alpha-beta adâncime 1 cu valori așteptate.

## Diferențe față de regulile tradiționale

Joc original („scrise de noi”), fără abateri declarate față de un joc tradițional.
