# Pai Gow (domino chinezesc) (`paigow`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play paigow --seed 1`. Sursa implementării: `polytabletop-games/src/pai_gow.rs`.

> Rezumat din `polytabletop list`: 32 de piese; 4 piese fiecare, împărțite în față (mâna mică) și spate (mâna mare); față vs față și spate vs spate contra dealerului (rolul alternează; copiile și 0–0 merg la dealer). Perechi (Gee Joon > Teen > Day > … > Chop-Ng) > Wong > Gong > puncte modulo 10. Ambele câștigate = +10, una și una = remiză. 12 runde, 100 de jetoane. Mutări: aranj 1/2/3, arata, urmatoarea.

## Scop

Câștigi ambele mâini (față și spate) contra dealerului în cât mai multe runde.

## Pregătire

32 de piese de domino chinezesc. Fiecare primește 4 piese și le împarte în două mâini de câte 2: fața (mâna mică) și spatele (mâna mare). Dealerul alternează (J2 e dealer în prima rundă). Fiecare rundă se joacă pe 10 jetoane din 100.

## Desfășurarea turei

Alegi una dintre cele 3 aranjări posibile cu `aranj N`; mâna mai tare devine spatele. Apoi `arata` compară față cu față și spate cu spate; `urmatoarea` trece la runda următoare.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `aranj 1`, `aranj 2`, `aranj 3`.
* `arata`, `urmatoarea` (forțate).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (3): `aranj 1`, `aranj 2`, `aranj 3`. Introduci `aranj 1`; răspunsul calculatorului (în ordine): `aranj 3`.
2. Mutări legale (1): `arata`. Introduci `arata`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (1): `urmatoarea`. Introduci `urmatoarea`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Ierarhie: Gee Joon > perechi (Teen > Day > Yun > Gor > Mooy > Chong > Bon > Foo > Ping > Tit > Look > Chop-Gow, Chop-Bot, Chop-Chit, Chop-Ng) > Wong > Gong > puncte modulo 10.
* La puncte egale decide cea mai mare piesă; la egalitate perfectă (copie) și la 0–0 câștigă dealerul.
* Una și una = remiză; ambele câștigate = +10.

## Punctaj

Jetoane: 10 pe rundă, din 100 la start.

## Sfârșitul jocului

După 12 runde, sau dacă cineva rămâne fără jetoane; câștigă cine are mai multe jetoane.

## Note despre AI

Informație ascunsă: vezi doar piesele tale; aranjarea nu e arătată adversarului până la `arata`. AI-ul își alege aranjarea doar din propriile piese. Împărțirea este automată.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Regula „foul” (spatele trebuie să fie ≥ fața) este aplicată automat prin alegerea mâinii mai tari ca spate.
