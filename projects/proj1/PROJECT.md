# Laboratorium 1 - Pomiary podstawowych wielkości przy odczytach plików

- [Laboratorium 1 - Pomiary podstawowych wielkości przy odczytach plików](#laboratorium-1---badanie-sposobów-odczytu-pliku)
  - [Strategie czytania pliku](#strategie-czytania-pliku)
  - [Operacja na wczytanych danych](#operacja-na-wczytanych-danych)
  - [Pomiar czasu](#pomiar-czasu)
  - [Różne technologie dyskowe](#różne-technologie-dyskowe)
  - [Materiały do przeczytania](#materiały-do-przeczytania)


Celem laboratorium jest zbadanie 2 wielkości związanych z oprogramowaniem zajmującym się odczytami z dysków:
* latencja odczytu
* przepustowość transmisji danych
Systemy DBMS typu OLPT (latencja) oraz OLAP (przepustowość) budowane są optymalanie pod jedną z tych wielkości.


Twoim zadaniem w ramach tego laboratorium będzie poprawienie danych programów pod wybranym kątem.
Podstawowy porgram wykonany jest w języku Rust i w tym także języku sugeruję oddawać rozwiązanie tego laboratorium.
Jest to jednak jedynie sugestia. MOżna oddawać rozwiązanie w innych językach.

## Problem - Obliczenie sumy XOR funkcji skrótu bloku z każdego pliku

Wszystkie poniższe programy będą dzieliły program na bloki o rozmiarze `M` bajtów i ich celem jest obliczenie funkcji skrótu każdego z bloków.
Wynikiem działania porgramu powinna być suma XOR każdego z tych skrótów.
Taka operacja jest przemienna i łączna, co daje nam taki sam wynik niezależnie od kolejności odczytanych bloków w pliku.

TODO: dodac odpowiedni rysunke tlumaczacy czym są bloki.
![strategies](res/strategies.svg)

$$
\begin{aligned}
R(F) = H(B_1) \oplus H(B_2) \oplus \dots \oplus H(B_N)
\end{aligned}
$$,
gdzie $N$ to rozmiar pliku $F$ podzielony na $M$ oraz $B_i$ to ciągły kawałek pliku rozpoczynający się na bajcie $i \cdot M$ i kończący się na $(i + 1)M - 1$. 

Aby dodać parametr sterujący ilościa obliczeń programu funkcja hashująca to wielokrotne wykonanie funkcji MD5 na wyjściu poprzedniej iteracji.

$$
\begin{aligned}
H(B,K) = \underbrace{MD5(MD5( \dots MD5(B)))}_\text{K razy}
\end{aligned}
$$
Ta technika to tzw. *Key Stretching*. Pozwala on parametryzować jak trudno jest wykonać atak brute-force na hash zwiększając wartość $K$. 

W taki sposób problem ma dwie cześci:
1. *read* - operacja przecyztania bloku z dysku (*producent*)
2. *compute* - operacja wyliczenia funkcji $H(B_i)$ (*konsument*)

## Programy do poprawienia
### Latencja - losowy odczyt

Pierwszy program rozwiązuje problem czytając bloki w losowej kolejności.
Używa on do tego synchronicznego `read()` i małych bloków ($M \approx 512B$).
W tym momencie czas wykonania obliczeń to mniej więcej $T \approx N * latency$.

Parametr $K=1$ jest ustalony na stałe.
Rozmiar bloku $M$ jest parametrem programu.
Każda operacja *read* potrzebuje czasu na wysłanie zlecenia i wysłanie danych do procesora.
Operacja *compute* jest pomijalnie krótka względem czytania z dysku.

Popraw ten program dodając wiele wątków, które wykonują operacje `pread` z wcześniej obliczonym offsetem.
Każdy wynik przekaż do jedengo wątku wykonujący operację *compute*.

Zbadaj czas $T$ wykonania programu dla różnych M i rożnej ilości wątków.

### Przepustowość - mechanizm zero-copy

Drugi program rozwiązuje problem czytając plik sekwencyjnie od poczatku do końca.
Używa on do tego synchronicznego `read()` i małych bloków ($M \approx 512B$).
W tym momencie czas wykonania programu to w przepustowość dysku oraz czas przejścia w tryb kernela (syscall) oraz kopiowanie danych do przestrzeni pamięci procesu.

Parametr $K=1$ jest ustalony na stałe.
Rozmiar bloku $M$ jest parametrem programu.
Każda operacja read wymaga nie tylko transmisji danych z dysku ale także przejścia przez system operacyjny i skopiowanie danych z przestrzeni kernela

Popraw ten program mapując plik do pamięci wirtulanej procesu.
Użyj do tego funkcji *mmap()*.

Zbadaj czas wykonanaia programu dla różnych M.
Jak abędzie róznica czasu wykonania przy pliku już znajdujacym się w pamięcy systemu? (file cache)

### Przeplot obliczeń oraz operacji IO - asynchorniczne operacje IO

Trzeci program rozwiązuje problem także czytająć sekwencyjnie, jednak tym razem K jest duże ($K=1000).
Używa on do tego synchronicznego `read()` i średnich bloków ($M \approx 1M$).
W tym scenariuszu czas wykonania programu to suma operacji IO (potencjalna latencja i przepustowość) oraz czas wykonania operacji *compute*.

Rozmiar bloku $M$ oraz ilośc itearacji $K$ są parametrami programu.
Popraw powyzszy program tworząc dwa wątki: czytający z pliku oraz obliczający funkcję skrótu.
Połącz wątki kanałem o ograniczonej wielkości (`sync_channel`).

W tej architekturze czas wykonania zmieni się z sumy czasów *read* oraz *compute* na maximum z dwóch.
Generalnie dla małych K, dysk jest wolnieszy niz obliczanie funkcji skrótu.
Oznacza to, że istnieje takie $K_{opt}$, gdzie w zakresie $[K, K_{opt}] program nie zwalnia.
Natomiast każde $K > K_{opt}$ powoduje zwiększenia czasu działania.

Zbadaj czas wykonania dla róznych $K$ i znajdz $K_{opt}$ dla twojego komputera i ustalonego $M$ oraz pliku.
Czy dodanie większej ilości wątków po stronie czytania lub funkcji hashującej zmieni cokolwiek?

## Jak wykonać pomiary?

### Pomiar czasu

### Mechanizm file cache

### Dobór rozmiaru plików

## Sprawozdanie z pomiarów


## Strategie czytania pliku
Typowo dzieli się pliki na bloki stałego rozmiaru (np. 8M), aby ułatwić zarządzanie odczytanymi wcześniej częściami pliku.
Po takim podziale należy zaimplementować dwie strategie czytania plików:
* sekwencyjną
* losową (dla uproszczenia plik będzie czytany od tyłu i od przodu na zmianę).

Poniżej można zobaczyć wizualizacje czytania plików powyższymi strategiami w przypadku podzielenia pliku na bloki.

![strategies](res/strategies.svg)

## Operacja na wczytanych danych

Aby upewnić się, że dane zostały wczytane prawidłowo program powinien wyliczyć hash CRC64 z pliku wczytanego każdą strategią i sposobem.
Funkcja skrótu jest przemienna, więc suma powinna zgadzać się dla każdej strategii odczytu.
Implementacja algorytmu jest poza zakresem tego laboratorium, więc można skorzystać z wybranej implementacji algorytmu z internetu (oczywiście proszę uważać na licencję!)

## Pomiar czasu
W ramach projektu interesuje nas pomiar czasu operacji wczytania całego pliku oraz obliczenia funkcji CRC64.
Operacje tworzenia zasobów (otwierania pliku lub mapowania go do pamięci dzielonej) można pominąć.
Za pomocą funkcji `clock_gettime()` można uzyskać pomiar czasu z dokładnością do nanosekund.

Ostatecznie program powinien wypisać na standardowe wyjście 4 pomiary i 4 hashe:
1. Sekwencyjny odczyt za pomocą funkcji `read()`
2. Losowy odczyt za pomocą funkcji `read()`
3. Sekwencyjny odczyt z mapowanego pliku za pomącą funkcji `mmap()`
4. Losowy odczyt z mapowanego pliku za pomącą funkcji `mmap()`

Program powinien być w stanie przeczytać plik o rozmiarze większym niż dostępna pamięć operacyjna.

*Hint: polecam stworzyć pliki różnej wielkości do testowania programu*

## Różne technologie dyskowe

Na laboratorium nr 3 wykonamy test wybranych programów na trzech różnych technologiach dysków:
* Dysk talerzowy (HDD)
* Dysk SATA SSD
* Dysk NVMe SSD

## Materiały do przeczytania
* Instrukcje użytkownika POSIX(3p) oraz syscalle(2)
  * `read(2)` + `read(3p)`
  * `open(2)`
  * `fseek(3p)`
  * `fstat(3p)`
  * `mmap(2)` + `mmap(3p)`
  * `munmap(3p)`
  * `msync(3p)`
  * `sys_mman.h(0p)`
  * `clock_gettime(3p)`
