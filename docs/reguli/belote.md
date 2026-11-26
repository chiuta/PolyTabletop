# Belote (Franța, 2 jucători) (`belote`)

Belote adaptat pentru doi jucători, cu pachet de 32 de cărți și licitație în două tururi.

## Scop

Aduni mai multe puncte decât adversarul în fiecare mână și ajungi primul la 301 de puncte în meci.

## Pregătire

32 de cărți (7 8 9 10 J Q K A; c d h s). Fiecare primește 5 cărți, o carte se întoarce (propunerea de atu). Cine ia primește cartea întoarsă și se împarte până la 8 cărți fiecare (mutarea forțată `imparte`); cele 16 cărți rămase nu se joacă niciodată.

## Desfășurarea unei ture

Licitație: în turul 1 fiecare spune `prends` (atu = culoarea întoarsă) sau `pas`; în turul 2 `prends-c|d|h|s` (altă culoare) sau `pas`. Dacă toți dau pas, se redistribuie. Apoi se joacă 8 levate: se răspunde la culoare; dacă nu ai culoarea tai obligatoriu cu atu (dacă ai); la atu urci peste atuul de pe masă. Câștigătorul unei levate conduce. La sfârșitul mâinii tastezi `urmatoarea`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `prends` — iei cartea întoarsă; atu = culoarea ei
* `pas` — refuzi
* `prends-h` — în turul 2: atu cupă (h); alternative `prends-c`, `prends-d`, `prends-s`
* `imparte` — mutare forțată: se completează mâinile
* `Jh` — joci Valetul de cupă; rang + culoare, ex. `10s`
* `urmatoarea` — începe mâna următoare

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play belote --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `prends` — iei cartea întoarsă ca atu.
2. `imparte` — `imparte` completează mâinile până la 8 cărți.
3. `7d` — primul atu al levatei: 7 de caro.

## Reguli speciale și variante alese

Ordinea în atu: J 9 A 10 K Q 8 7 (puncte J 20, 9 14, A 11, 10 10, K 4, Q 3); în alte culori A 10 K Q J 9 8 7 (A 11, 10 10, K 4, Q 3, J 2). Bonusuri: belote-rebelote (K și Q de atu în aceeași mână) 20, dix de der (ultima levată) 10, capot (toate cele 8 levate) 90. Nu există partener.

## Punctaj

Cine a luat trebuie să ajungă cel puțin la punctele adversarului; dacă are mai puține, contractul „cade” și adversarul scrie toate punctele din mână (belote rămâne cui îl are).

## Sfârșitul jocului

Meciul se termină la 301 de puncte.

## Note despre AI și informația ascunsă

Informație ascunsă: se vede doar mâna Jucătorului 1; cele 16 cărți nejucate rămân ascunse. AI-ul (alpha-beta adâncime 1) judecă licitația doar după cele 5 cărți ale lui și cartea întoarsă. Împărțirea completă (`imparte`) este mutare forțată.

## Diferențe față de regulile tradiționale

Variantă „belote clasică” adaptată la doi: fără partener, 16 cărți nejucate (nu se poate număra tot pachetul), fără anunțuri de serii (tierce, quarte) și fără dublare.
