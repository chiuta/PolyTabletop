# Cetățuia (muncitori pe drum de clădiri, castel) (`cetatuia`)

Cetățuia: joc de plasare de muncitori pe un drum de clădiri și construire a unei cetăți, 2 jucători, informație completă.

## Scop

Aduni mai multe puncte: livrări la castel, clădiri construite, majoritatea pieselor.

## Pregătire

Resurse: bani, lemn, piatră, hrană. Încep cu 5 bani, 2 lemn, 1 piatră, 2 hrană; fiecare are 5 muncitori; în fiecare rundă se primesc 3 bani. Drumul pornește cu 8 locuri: `piata` (+3 bani), `ferma` (+2 hrană), `padure` (+2 lemn), `cariera` (+2 piatră), `atelier` (clădire de lemn), `arhitect` (clădire de piatră), `castel`, `pod`. Clădirile construite se adaugă la capătul drumului.

## Desfășurarea unei ture

Pe rând plasezi un muncitor pe un loc liber (`pune ferma`); primele 3 plasări ale rundei costă 1 ban, următoarele 2. `pas` = nu mai plasezi în runda asta; primul care dă pas primește 1 ban și începe runda următoare. La `pod` muncitorul mută jalonerul cu 1–3 locuri (`pune pod +2`, `pune pod -1`), plătind 1 ban pe loc. Când ambii au terminat, locurile se activează în ordine, până la jaloner inclusiv.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `pune piata` — muncitor pe piață (și ferma, padure, cariera, atelier, arhitect, castel)
* `pune pod -1` — muncitor pe pod, jalonerul cu 1 loc înapoi
* `pas` — nu mai plasezi în runda aceasta
* `construieste gater` — la atelier/arhitect construiești clădirea gater
* `sari` — la atelier/arhitect nu construiești
* `castel l2 p1` — livrezi 2 piese cu lemn, 1 cu piatră (fiecare piesă costă încă 1 hrană; cel mult 3 piese)

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play cetatuia --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `pune piata` — muncitor pe piață (+3 bani).
2. `pune pod -1` — muncitor pe pod: jalonerul se mută cu 1 loc înapoi.
3. `pas` — pas: nu mai plasezi în runda asta.

## Reguli speciale și variante alese

Efectele fără plată se aplică singure; clădirile cu plată se activează doar dacă le poți plăti. Când folosești clădirea altuia, proprietarul primește 1 punct. Jalonerul avansează cu un loc la sfârșitul fiecărei runde.

## Punctaj

Cetatea are 12 piese: primele 4 valorează 3 puncte, următoarele 4 câte 2, ultimele 4 câte 1. Clădirile construite aduc 2 (lemn) sau 3 (piatră) puncte. La final: cine a livrat strict mai multe piese primește 4 puncte; câte 1 punct pentru fiecare 4 resurse rămase.

## Sfârșitul jocului

După runda în care cetatea e completă sau după runda 9.

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul (alpha-beta adâncime 6) vede toată starea.

## Diferențe față de regulile tradiționale

Joc original (mecanică generică); nu se declară alte abateri.
