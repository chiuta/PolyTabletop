# Spionii Cuvintelor (grilă 5×5) (`spionii`)

Spionii Cuvintelor: variantă proprie de joc de spionaj cu cuvinte, grilă 5×5. Maestrul-spion al fiecărei echipe este calculatorul; tu ești ghicitorul.

## Scop

Descoperă-ți toți agenții (9 pentru roșii = J1, 8 pentru albaștri = J2) înaintea adversarului, fără să atingi asasinul.

## Pregătire

Grilă de 25 de cuvinte: 9 agenți roșii, 8 albaștri, 7 neutre, 1 asasin. Maestrul-spion (calculatorul) dă ca indiciu numele unei teme din tabela proprie (ex. MARE, CIRC) și un număr; alege tema care acoperă cât mai mulți agenți nedescoperiți ai lui și cât mai puțin din ceilalți.

## Desfășurarea unei ture

Jucătorul la mutare este ghicitorul echipei: `ghici cuvânt` (fără diacritice), apoi pasul forțat `dezvaluie` întoarce cartea. Poți ghici cel mult „număr + 1” cuvinte; `stop` încheie tura (după cel puțin o ghicire). Cuvânt al echipei tale → continui; neutru → tura trece; al adversarului → tura trece (el e mai aproape de victorie); asasin → pierzi imediat.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `ghici oglinda` — ghicești cuvântul OGLINDĂ (fără diacritice)
* `stop` — încheie tura
* `indiciu` — pas forțat: maestrul-spion dă indiciul
* `dezvaluie` — pas forțat: se întoarce cartea

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play spionii --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `ghici oglinda` — ghicești OGLINDĂ (aici: agent roșu).
2. `stop` — `stop` încheie tura.
3. `ghici pachet` — în tura următoare ghicești PACHET (agent roșu).

## Reguli speciale și variante alese

Maestrul-spion nu folosește o temă care conține asasinul decât dacă nu are alternativă.

## Punctaj

Afișajul arată agenții rămași de găsit pentru fiecare echipă.

## Sfârșitul jocului

Câștigă cine își descoperă toți agenții; asasinul pierde pe loc.

## Note despre AI și informația ascunsă

AI-ul (ca ghicitor) folosește aceeași tabelă de teme ca maestrul-spion (un „cod” comun); nu vede cheia: valoarea unei mutări depinde doar de indiciu și de cuvinte.

## Diferențe față de regulile tradiționale

Variantă originală a mecanicii publice a genului; cuvintele și temele sunt ale noastre; maestrul-spion este mereu calculatorul.
