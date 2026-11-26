# Oware (Ghana) (`oware`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play oware --seed 1`. Sursa implementării: `polytabletop-games/src/oware.rs`.

> Rezumat din `polytabletop list`: 2 × 6 gropi, 4 semințe în fiecare; semănat trigonometric (groapa de start e sărită la 12+ semințe), fără lanț. Captură: dacă ultima sămânță cade într-o groapă adversă ajunsă la 2 sau 3 semințe, le iei și continui înapoi cât timp grupele au 2 sau 3; mutarea care ar goli complet partea adversă nu capturează (grand slam). Dacă adversarul nu are semințe, trebuie să-l hrănești. Câștigă primul cu 25. Mutări: 1–6.

## Scop

Capturezi 25 de semințe (din 48).

## Pregătire

2 × 6 gropi, 4 semințe în fiecare. Jucătorul 1 are gropile 1–6 în rândul de jos (stânga → dreapta), Jucătorul 2 în rândul de sus (dreapta → stânga).

## Desfășurarea turei

Alegi o groapă proprie nevidă și semeni în sens trigonometric, câte o sămânță în fiecare groapă; dacă groapa are 12 sau mai multe semințe, ea este sărită. Nu există semănat în lanț.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Numărul gropii proprii: `1`–`6`.

## Reguli speciale și variante alese

* Captură: dacă ultima sămânță cade într-o groapă ADVERSĂ care ajunge la 2 sau 3 semințe, le iei; apoi te uiți la groapa dinainte (tot a adversarului) și continui cât timp are 2 sau 3.
* Grand slam: o mutare care ar lua toate semințele adversarului este legală, dar nu capturează nimic.
* Hrănire: dacă adversarul nu are semințe, ești obligat să-i dai; dacă nu poți, jocul se termină și îți iei semințele de pe partea ta.

## Punctaj

Semințele capturate.

## Sfârșitul jocului

Primul care ajunge la 25 câștigă; la 24–24 e remiză. Dacă la mutare nu mai ai semințe, adversarul le ia pe ale lui. După 400 de mutări fiecare ia semințele de pe partea lui.

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă 7.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar nu are informație ascunsă.

## Diferențe față de regulile tradiționale

* Oware este un motor separat de `ayo` (acela folosește semănat în lanț și captură pe partea proprie); regulile nu coincid, deci nu s-a reutilizat codul.
* Limita de 400 de mutări înlocuiește regula de repetiție de poziții.
