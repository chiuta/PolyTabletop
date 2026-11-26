# Teen Patti (heads-up) (`teenpatti`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play teenpatti --seed 1`. Sursa implementării: `polytabletop-games/src/teen_patti.rs`.

> Rezumat din `polytabletop list`: 3 cărți fiecare, 12 mâini, 500 de jetoane, boot 10. Ierarhie: trio > scară de culoare > scară (AKQ, A23, KQJ…) > culoare > pereche > carte mare. Mutări: pack (abandon), chaal (plătești miza), raise (dublezi, max 80), show (după cel puțin o acțiune; la egalitate pierde cel care cere show); arata și urmatoarea sunt forțate. Maxim 6 acțiuni pe mână.

## Scop

Ai mai multe jetoane după 12 mâini.

## Pregătire

Pachet de 52, 3 cărți fiecare. Fiecare jucător pornește cu 500 de jetoane; la fiecare mână se plătește „boot” 10, iar miza (chaal) pornește de la 10. Toți joacă „văzut”.

## Desfășurarea turei

La mutare: `pack` (abandon), `chaal` (plătești miza curentă), `raise` (dublezi miza; cel mult 80) sau `show` (plătești miza și se compară mâinile; după cel puțin o acțiune). Cel mult 6 acțiuni pe mână, apoi doar `pack` sau `show`. Un raise e permis doar dacă amândoi pot plăti până la 3 ori noua miză.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `pack`, `chaal`, `raise`, `show`.
* `arata` și `urmatoarea` (forțate).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (3): `pack`, `chaal`, `raise`. Introduci `pack`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1): `urmatoarea`. Introduci `urmatoarea`; răspunsul calculatorului (în ordine): `raise`.
3. Mutări legale (4): `pack`, `chaal`, `raise`, `show`. Introduci `show`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Ierarhie: trio > scară de culoare > scară (A-K-Q cea mai mare, apoi A-2-3, apoi K-Q-J … 4-3-2) > culoare > pereche > carte mare.
* La egalitate perfectă pierde cel care a cerut `show`.

## Punctaj

Jetoanele câștigate din pot.

## Sfârșitul jocului

După 12 mâini sau când cineva rămâne cu mai puțin de 40 de jetoane; câștigă cine are mai multe jetoane.

## Note despre AI

Informație ascunsă; `arata` și `urmatoarea` sunt forțate ca AI-ul să nu vadă cărțile adversarului sau pachetul. AI-ul (adâncime 1) calculează valoarea așteptată din propria mână (percentila ei între cele 22 100 de mâini) și din pariuri; nu bluffează.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Variantă heads-up simplificată; fără joc „orb”.
