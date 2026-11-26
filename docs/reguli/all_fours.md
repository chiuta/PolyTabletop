# All Fours (`all_fours`)

All Fours (Old Sledge, Trinidad) pentru doi jucători.

## Scop

Primul la 14 puncte.

## Pregătire

52 de cărți, 6 fiecăruia; o carte se întoarce și culoarea ei e atu (Valet întors = 3 puncte dealerului).

## Desfășurarea unei ture

Non-dealerul `cer` sau `joc`. La `cer`, dealerul `dau` (un punct) sau `schimb`: se dau încă 3 cărți (`imparte`), se întoarce alt atu, apoi fiecare aruncă până la 6 (`dis 5h`). Non-dealerul conduce; răspunzi la culoare sau tai cu atu.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Exemple de mutări folosite de această implementare:

* `cer`
* `dau`
* `imparte`
* `dis 5h`
* `Ah`
* `urmatoarea`

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Patru puncte: High și Low către cel care le-a avut, Jack către cel care ia levata cu el, Game (A=4, K=3, Q=2, J=1, 10=10). Omise: hang jack și kick pentru As/Șase (variante regionale).
