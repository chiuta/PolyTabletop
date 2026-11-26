# Senet (`senet`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play senet --seed 1`. Sursa implementării: `polytabletop-games/src/senet.rs`.

> Rezumat din `polytabletop list`: Cursă egipteană pe 30 de pătrate (reconstituirea lui Kendall), 5 piese pe jucător, 4 bețe (1–4, zero = 5; 1, 4, 5 dau aruncare în plus). Schimbi locul cu o piesă adversă neprotejată; nu treci de 26; 27 (apa) te trimite pe 15; ieșire din 26 cu 5, din 28 cu 3, din 29 cu 2, din 30 cu 1. Mutări: arunca, 7-9, 28-iese, pas.

## Scop

Scoți primul toate cele 5 piese.

## Pregătire

30 de pătrate parcurse în S. Jucătorul 1 pe 1, 3, 5, 7, 9; Jucătorul 2 pe 2, 4, 6, 8, 10. Se aruncă 4 bețe cu două fețe.

## Desfășurarea turei

Aruncarea dă 1–4 (fețe marcate în sus) sau 5 (niciuna); 1, 4 și 5 dau o nouă aruncare. O piesă înaintează exact cât arată aruncarea; nu aterizezi pe o piesă proprie; aterizarea pe una adversă neprotejată o schimbă cu a ta. Protejate sunt piesele vecine cu alta a aceluiași adversar și cele de pe 15, 26, 28, 29. Nu poți trece peste trei piese adverse consecutive.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `arunca`.
* `7-9`: piesa de pe pătratul 7 merge pe 9; `28-iese`: ieșire.
* `pas` dacă nu ai nicio mutare.

## Reguli speciale și variante alese

* Pătratul 26 (Casa Fericirii): nu se trece de el fără oprire; ieșire din 26 cu 5.
* Pătratul 27 (apa): piesa revine pe 15 (sau pe primul pătrat liber înainte de 15).
* Din 28 se iese cu 3, din 29 cu 2, din 30 cu 1 (se poate și avansa cu aruncări mai mici).

## Punctaj

Piesele scoase.

## Sfârșitul jocului

Câștigă primul care scoate toate cele 5 piese.

## Note despre AI

Joc cu zaruri: AI-ul alege după aruncare, cu adâncime 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar nu are informație ascunsă.

## Diferențe față de regulile tradiționale

* Nu există reguli originale atestate; se folosește reconstituirea lui Kendall (cea mai răspândită), cu simplificările de mai sus.
