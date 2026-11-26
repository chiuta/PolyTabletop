# Hwatu (Go-Stop) (`hwatu`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play hwatu --seed 1`. Sursa implementării: `polytabletop-games/src/hwatu.rs`.

> Rezumat din `polytabletop list`: Aceleași 48 de cărți de flori; 10 în mână, 8 pe masă, 20 în stoc. Captură ca la Hanafuda (3c, 3a:3c, intoarce, ia 3c). Scor: gwang, godori, hongdan/cheongdan/chodan, tti, yeolkkeut, pi. De la 7 puncte alegi stop (încasezi) sau go (continui: +1 la primele două, apoi ×2; ×2 gwang-bak, pi-bak și go-bak). Măturarea mesei ia o simplă de la adversar. 3 runde. Fără ppeok/jjok/ttadak.

## Scop

Aduni puncte prin combinații și, de la 7 puncte, alegi `stop` sau `go`; câștigă cine are mai multe puncte după 3 runde.

## Pregătire

Aceleași 48 de cărți de flori ca la Hanafuda; 10 în mână, 8 pe masă, 20 în stoc.

## Desfășurarea turei

Joci o carte din mână (captură obligatorie dacă are pereche pe masă; doi candidați = alegi, trei = îi iei pe toți), apoi `intoarce` din stoc și joci la fel (`ia 3c` la alegere).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Carte: `3c`; captură la alegere: `3a:3c`.
* `intoarce` (forțată), `ia 3c`.
* `go` (continui) sau `stop` (încasezi), după ce ai ajuns la 7 puncte.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (10): `1b`, `3c`, `3d`, `4c`, `4d`, `6b`, `6c`, `6d`, `9c`, `11d`. Introduci `1b`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1): `intoarce`. Introduci `intoarce`; răspunsul calculatorului (în ordine): `3b`; `intoarce`.
3. Mutări legale (9): `3c`, `3d`, `4c`, `4d`, `6b`, `6c`, `6d`, `9c`, `11d`. Introduci `11d`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Punctaj: Gwang 3 luminoase = 3 (2 dacă include omul-ploii), 4 = 4, 5 = 15; Godori 5; Hongdan 3; Cheongdan 3; Chodan 3; Tti (5+ panglici) 1 + 1 în plus; Yeolkkeut (5+ animale) 1 + 1; Pi (10+ simple) 1 + 1.
* Go: baza + 1 la primul și al doilea go; de la al treilea punctele se dublează de fiecare dată; ×2 gwang-bak, ×2 pi-bak, ×2 go-bak.
* Măturarea mesei îți aduce o carte simplă de la adversar.

## Punctaj

Punctele combinațiilor, cu plata de mai sus.

## Sfârșitul jocului

Meci de 3 runde; câștigă cine are mai multe puncte. Mâinile golite fără `stop` = remiză în acea rundă.

## Note despre AI

Informație ascunsă (mână, stoc). Acest joc NU este în lista `HOTSEAT_SAFE`. AI-ul (adâncime 1) spune `stop` imediat ce ajunge la 7 puncte.

Stil AI în cod: .

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Fără ppeok, jjok, ttadak, bombă și shaking; cupa de sake e doar animal; fără „meongtta”.
