# Miniaturi hexagonale (Trecătoarea Cenușie) (`miniaturi`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play miniaturi --seed 1`. Sursa implementării: `polytabletop-games/src/miniaturi.rs`.

> Rezumat din `polytabletop list`: Motor generic de miniaturi pe hartă hexagonală: 7 unități originale pe parte (Lăncieri, Arcași, Călăreți, Colos, Catapulte, Comandant), teren (stâncă blochează vederea, pădure = acoperire), zaruri de lovire și salvare, moral (șovăit, apoi fugă). 6 runde cu activări alternante; 3 obiective (cine are mai multe figuri la ≤1 hex ia 1 punct pe rundă); eliminarea armatei adverse câștigă. Informație completă; zarurile se aruncă la mutarea forțată arunca. Mutări: A pas, A>d4, A>d4 xK (mută și atacă), A xK, arunca.

## Scop

Aduni cele mai multe puncte de victorie (obiective) în 6 runde sau distrugi toată armata adversă.

## Pregătire

Hartă hexagonală de 9 rânduri (rândurile pare 11 hexuri, cele impare 10) cu teren: `#` stâncă (nu se trece, blochează linia de vizibilitate), `f` pădure (mișcare 2, acoperire: salvarea apărătorului +1), `O` obiectiv. Fiecare parte are 7 unități originale (Lăncieri ×2, Arcași, Călăreți, Colos, Catapulte, Comandant). Unitățile au etichete A–G (J1) și H–N (J2).

## Desfășurarea turei

O rundă: jucătorii activează pe rând câte o unitate (J1 începe rundele impare, J2 pe cele pare); fiecare unitate se activează o dată pe rundă. O activare = mutare opțională (cost de mișcare ≤ mv; nu treci prin alte unități; intrarea lângă un inamic oprește mișcarea) + o acțiune opțională: foc (în rază, cu linie de vizibilitate liberă, nu cât ești lipit de un inamic) sau luptă corp la corp. Atacul are două trepte: declari, apoi mutarea forțată `arunca` aruncă zarurile.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Doar activare/pas: `B pas` (unitatea B nu face nimic); mutare: `B>d4`; mutare + atac: `B>d4 xK`; atac fără mutare: `B xK`.
* `arunca` (mutare forțată după un atac declarat).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (187): `A pas`, `A>c1`, `A>e1`, `A>b2`, `A>c2`, `A>d3`, `A>c4`, `A>d4`, `A>e4`, `A>c5` …. Introduci `A pas`; răspunsul calculatorului (în ordine): `I>e5`.
2. Mutări legale (140): `B pas`, `B>g1`, `B>h1`, `B>i1`, `B>j1`, `B>f2`, `B>h2`, `B>i2`, `B>f3`, `B>g3` …. Introduci `D>k3`; răspunsul calculatorului (în ordine): `H>h5`.
3. Mutări legale (106): `B pas`, `B>g1`, `B>h1`, `B>i1`, `B>j1`, `B>f2`, `B>h2`, `B>i2`, `B>f3`, `B>g3` …. Introduci `G>g4 xH`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Zaruri: ataci cu (figuri × zaruri/figură) d6; lovești la ≥ ținta de lovire (foc la peste jumătate din rază +1; Călăreți care au mutat ≥ 3: −1 la corp la corp; Lăncieri contra Călăreți −1, Călăreți contra Lăncieri +1). Fiecare lovitură e salvată de un d6 ≥ salvarea apărătorului (acoperire −1). Fiecare lovitură nesalvată elimină o figură; supraviețuitorul lovește înapoi la corp la corp.
* Moral: o unitate care pierde figuri dă un d6; peste prag devine Șovăitoare (nu poate ataca, nu contează la obiective); a doua cădere o pune pe fugă (distrusă). Șovăitoarele își revin la sfârșitul rundei dacă nu au inamic alăturat.
* Catapultele (grele) mută SAU trag.

## Punctaj

La sfârșitul rundei, fiecare obiectiv aparține celui cu mai multe figuri nesovăitoare la ≤ 1 hex: 1 punct de victorie.

## Sfârșitul jocului

După 6 runde: câștigă cine are mai multe puncte, apoi mai mult efectiv rămas. Distrugerea întregii armate adverse câștigă imediat.

## Note despre AI

Informație completă; zarurile sunt aruncate la mutarea forțată `arunca` cu generatorul propriu al stării (la `clone()` zarurile își schimbă sămânța, deci o căutare nu poate „ghici” aruncarea reală). AI-ul (adâncime 1) evaluează cu valoarea AȘTEPTATĂ a atacului declarat, calculată exact.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Motor generic cu unități și reguli originale; nu reproduce un joc comercial cu miniaturi.
