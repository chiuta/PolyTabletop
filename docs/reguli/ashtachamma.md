# Ashta Chamma (`ashtachamma`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ashtachamma --seed 1`. Sursa implementării: `polytabletop-games/src/ashtachamma.rs`.

> Rezumat din `polytabletop list`: Tablă 5×5, 4 piese pe jucător, 4 cauri (1–4; zero în sus = 8; 4 și 8 dau încă o aruncare). Piesele merg în sens trigonometric pe inelul exterior (16 pătrate) și intră în inelul interior și în centru doar după o captură; centrul se atinge exact. Pătrate sigure (X): mijloace de latură, colțuri, centru; două piese ale aceluiași jucător formează un bloc. Captura dă încă o aruncare. Câștigă cine aduce toate piesele în centru. Mutări: arunc, 7 sau 7* (piesa de pe pătratul 7 al căii; * = a capturat), pas.

## Reguli implementate, variantă aleasă și omisiuni

Se alege varianta cea mai răspândită; ce lipsește este menționat explicit mai jos (rânduri cu „Omis”).

Ashta Chamma (India) — joc de cursă cu cauri pe tabla 5×5, 2 jucători, 4 piese fiecare.

Reguli folosite (varianta cea mai răspândită, în adaptare pentru doi):
* Tabla are 25 de pătrate. Jucătorii pornesc de la mijlocul laturilor opuse (J1 jos, J2 sus).
Se aruncă 4 cauri: numărul de cauri cu deschiderea în sus dă mutarea (1, 2, 3, 4), iar
nicio gură în sus = 8. **4 și 8 dau o aruncare în plus** (la fel o captură sau o piesă
ajunsă în centru).
* Piesele merg în sens trigonometric pe inelul exterior (16 pătrate), iar o piesă poate
intra în inelul interior (8 pătrate, sens orar) și apoi în centru **doar dacă a capturat
cel puțin o piesă adversă** („kill”); până atunci se învârte pe inelul exterior. Centrul se
atinge cu aruncare exactă.
* Pătratele sigure (X): cele patru mijloace de latură și cele patru colțuri, plus centrul.
Pe un pătrat nesigur, o piesă singură a adversarului e capturată (se întoarce la start,
pierzându-și „kill”-ul); două sau mai multe piese ale aceluiași jucător pe același pătrat
formează un bloc și nu pot fi capturate.
* Câștigă cine aduce primul toate cele 4 piese în centru.
* Mutări: `arunc`, apoi numărul pătratului de pe cale al piesei mutate (0 = start,
1–15 = inelul exterior, 16–23 = inelul interior, 24 = centru); un `*` după număr = piesa
a capturat deja (ex.: `7`, `7*`). `pas` dacă nu ai nicio mutare.
* Simplificări: fără varianta cu patru jucători, fără reguli speciale de dublet.
* AI: nu vede aruncarea următoare (se aruncă la mutarea forțată `arunc`); la Hard/Expert
evaluează în așteptare peste distribuția aruncărilor adversarului.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Evenimentele de șansă (zaruri, împărțire, tragere) sunt mutări forțate.
