# Legiunea în retragere (`legiunea_in_retragere`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play legiunea_in_retragere --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Rămășițele unei legiuni se retrag peste un pod rupt. Fiecare erou (Căpitan / Medic) are 20 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/2), «arta» (costă 2 focus, minte: te vindeci cu 4 PV), «odihna» (+2 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: pricepere zaruri d6 și contează cel mai mare (la pricepere 0: două zaruri, cel mai mic); 6 = reușit (două de 6 = excelent), 4–5 = parțial; dificultate 4+ scade zarul cu 1. La parțial poți plăti („plata”) 1 PV și rezultatul devine reușit. Regulă specială: o lovitură de 3+ PV adaugă 1 stres țintei. Mutarea «tinta» înseamnă a înainta pe drum (1/2/3 pași după parțial/reușit/excelent); primul la 16 câștigă; piedicile și primejdiile îl lovesc pe cel din față; după 20 runde decide drumul parcurs. La 14 stres pierzi. Oboseală: de la runda 15 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, plata.

Evenimentele MJ-ului (d6): 1 = răgaz: amândoi −1 stres; 2 = spaimă: amândoi +1 stres; 3 = avânt: cel din urmă +1 progres/PV; 4 = izvor: amândoi +1 focus; 5 = șansă: cel din urmă +1 noroc; 6 = ajutor: cel din urmă +2 PV.

Fișe: Căpitan (forță/iscusință/minte 1/3/2) contra Medic (2/3/1); focus inițial 2, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Retreating Legion.
