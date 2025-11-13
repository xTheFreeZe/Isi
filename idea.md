// ===========================================
// == Isi Core Spezifikation (Core.isi)
// ===========================================
//
// Dies ist die Referenz für alle Syntax-Konstrukte
// und die "primitiven" Funktionen, die in der Host-Sprache
// (z.B. V, Rust, C++) implementiert werden müssen.
//
// 'Isi' ist "Expression-Oriented"; alles ist ein Ausdruck, der
// einen Wert zurückgibt.

// -------------------------------------------
// ## 1. Kommentare
// -------------------------------------------
// Kommentare beginnen mit '//' und gehen bis zum Ende der Zeile.


// -------------------------------------------
// ## 2. Primitive Datentypen (Literale)
// -------------------------------------------

100             // Integer
3.14159         // Float
"Hallo Welt"    // String (unveränderlich, UTF-8)
true            // Boolean
false           // Boolean
:ein_keyword    // Keyword (nützlich für Map-Keys, wie in Clojure/Ruby)


// -------------------------------------------
// ## 3. Der Binde-Operator (->)
// -------------------------------------------
// Es gibt nur *eine* Art, Namen zu binden. Standardmäßig unveränderlich (immutable).
// Syntax: <name> -> <wert-ausdruck>

pi -> 3.14
willkommen -> "Hallo"


// -------------------------------------------
// ## 4. Die Syntax-Konstrukte
// -------------------------------------------

// --- ( ... ) : Der Aufruf (Call) ---
// Führt Code aus. Das erste Element *muss* eine Funktion sein.
// (funktion arg1 arg2)
(plus 1 2) // Ergibt 3. 'plus' muss ein Primitiv oder eine gebundene Funktion sein.


// --- [ ... ] : Der Vektor (Daten-Literal) ---
// Eine geordnete, unveränderliche Liste von Werten.
// Wird *nicht* ausgeführt.
mein_vektor -> [1 2 (plus 1 1) :key]
// `mein_vektor` enthält: [1 2 (plus 1 1) :key]
// (Es enthält den *Ausdruck* (plus 1 1), nicht das Ergebnis 2)


// --- { ... } : Die Map (Daten-Literal) ---
// Eine unveränderliche Key-Value-Struktur.
// Muss eine gerade Anzahl von Elementen haben.
// Keys sind oft Keywords, können aber beliebige Literale sein.
meine_map -> {:name "Aura" :version 1.0}


// --- | ... | : Die Pipeline (Datenfluss) ---
// Für lesbare Daten-Transformationen. Robust gegen 'null'.
// | <input> (f1) (f2) |  entspricht  (f2 (f1 <input>))
| "hallo"
    (string_upper)  // -> "HALLO"
    (string_rev)    // -> "OLLAH"
| // Ergibt "OLLAH"


// --- ! ... ! : Der Effekt (Side Effect) ---
// *Strikte* Markierung für Nebeneffekte (IO, Mutability).
// Der Compiler *muss* sicherstellen, dass reine Funktionen
// keine '!' Blöcke aufrufen.
! (print "Ich habe einen Nebeneffekt!") !


// --- < ... > : Die Verzögerung (Lazy) ---
// Der Code wird *nicht* ausgeführt, sondern als "Promise" oder
// "Thunk" zurückgegeben.
teure_op -> < (lange_berechnung 1000) >
// 'teure_op' ist jetzt ein Lazy-Objekt.

// Um es auszuführen (muss 'force' ein Primitiv sein):
(force teure_op)


// --- ? ... ? : Der Match (Pattern Matching) ---
// Strikte und sichere Alternative zu if/else.
// ? <input> <pattern1> -> <result1> ... _ -> <default> ?
meine_zahl -> 2
? meine_zahl
    1       -> "Eins"
    2       -> "Zwei"
    _       -> "Andere" // '_' ist die Wildcard (Default)
? // Ergibt "Zwei"


// -------------------------------------------
// ## 5. Funktionsdefinition (Lambda-Syntax)
// -------------------------------------------
// Funktionen sind nur Daten. Sie werden mit der Vektor-Syntax `[ ]`
// für die Parameterliste und einem `( )` Block für den Body erstellt.

// ([param1 param2] (body ...))

add -> ([a b] (plus a b))

// Verwendung:
(add 10 20) // -> 30

// Funktionen sind "First-Class-Citizens":
math_ops -> [add (minus) (mul)] // Eine Liste von Funktionen


// ===========================================
// ## 6. Core Primitives (Liste für den Compiler)
// ===========================================
//
// Dies sind die Funktionen, die Sie in Ihrer Host-Sprache
// (V, Rust, C++) implementieren müssen. Isi kann ohne sie
// nicht existieren.

// --- 6.1 Arithmetik & Vergleiche ---
// (plus a b)       -> a + b
// (minus a b)      -> a - b
// (mul a b)        -> a * b
// (div a b)        -> a / b (Achtung: Fließkomma- oder Integer-Div?)
// (mod a b)        -> a % b
//
// (eq a b)         -> Wertgleichheit (z.B. 1 == 1.0)
// (is a b)         -> Strikte Referenz-/Typgleichheit
// (gt a b)         -> a > b (greater than)
// (lt a b)         -> a < b (less than)
// (gte a b)        -> a >= b
// (lte a b)        -> a <= b


// --- 6.2 Logik ---
// (and a b) // Muss "short-circuiting" sein!
// (or a b)  // Muss "short-circuiting" sein!
// (not a)


// --- 6.3 Datenstruktur-Erzeugung (dynamisch) ---
// (list a b c)     -> Erzeugt Vektor [a b c]
// (map k1 v1 k2 v2)  -> Erzeugt Map {:k1 v1 :k2 v2}


// --- 6.4 Datenstruktur-Zugriff (unveränderlich) ---
// (get coll key)   -> Holt Wert aus Map oder Vektor (z.B. (get [1 2] 0) -> 1)
// (assoc coll k v) -> Gibt *neue* Kopie von `coll` zurück, mit `k` auf `v` gesetzt
// (dissoc coll k)  -> Gibt *neue* Kopie ohne `k` zurück
// (count coll)     -> Gibt Länge zurück
// (first coll)     -> [0]
// (rest coll)      -> [1...]


// --- 6.5 Kontrollfluss-Primitives ---
// (apply func arg-vektor) -> (apply plus [1 2]) entspricht (plus 1 2)
// (force lazy-obj)        -> Führt ein < ... > Objekt aus
// (cond ...)              -> (Primitiv hinter dem ? ... ? Konstrukt)


// --- 6.6 Effekt-Primitives (IO) ---
// (print ...)      -> Gibt auf Konsole aus (gibt 'nil' oder 'Effekt-Token' zurück)
// (read_line)      -> Liest von Konsole
// (read_file path)
// (write_file path content)


// --- 6.7 Typen & Konvertierung ---
// (type_of x)      -> :int, :float, :string, :vector, :map, :function, :lazy
// (to_string x)
// (to_int s)

// --- 6.8 Fehlerbehandlung (Für Striktheit & Robustheit) ---
//
// Robuste Sprachen erzwingen eine explizite Fehlerbehandlung.
// Isi-Funktionen geben *niemals* `null` oder Exceptions zurück.
// Stattdessen geben sie ein "Result"-Objekt zurück, das
// per Konvention eine Map ist:
// Erfolg: {:ok wert}
// Fehler: {:err "Fehlermeldung"}
//
// Beispiel: (div 10 0) -> {:err "Division durch Null"}
// Beispiel: (div 10 2) -> {:ok 5}
//
// Primitive, um dies zu unterstützen:
// (ok wert)      -> Erzeugt {:ok wert}
// (err msg)      -> Erzeugt {:err msg}
// (is_ok? x)     -> true wenn x ein :ok-Objekt ist
// (is_err? x)    -> true wenn x ein :err-Objekt ist
// (unwrap x)     -> Holt den Wert aus {:ok wert} oder crasht bei {:err ...} (für Tests!)
// (unwrap_err x) -> Holt die Meldung aus {:err msg}


// --- 6.9 Higher-Order Funktionen (Das Herz der Pipelines) ---
//
// Um die |...| Pipeline nützlich zu machen, brauchen wir
// Funktionen, die Funktionen auf Datenstrukturen anwenden.
//
// (map func coll)
// -> Wendet `func` auf jedes Element von `coll` an
// -> | [1 2 3] (map ([x] (plus x 1))) |  -> [2 3 4]
//
// (filter func coll)
// -> Behält nur Elemente, für die `(func element)` -> true ist
// -> | [1 2 3 4] (filter ([x] (eq (mod x 2) 0))) | -> [2 4]
//
// (reduce func init coll)
// -> Faltet die `coll` mit einem Akkumulator
// -> | [1 2 3] (reduce plus 0) | -> 6  (entspricht 0+1+2+3)


// --- 6.10 String-Operationen (Für Daten-Transformation/ETL) ---
//
// (string_join coll separator)
// -> | ["a" "b" "c"] (string_join ",") | -> "a,b,c"
//
// (string_split str separator)
// -> | "a,b,c" (string_split ",") | -> ["a" "b" "c"]
//
// (string_upper s)   -> "hallo" -> "HALLO"
// (string_lower s)   -> "HALLO" -> "hallo"
// (string_trim s)    -> "  hallo  " -> "hallo"
// (string_len s)     -> (Alternative zu (count s) für Strings)


// ===========================================
// ## 7. Metaprogrammierung (Das LISP-Erbe)
// ===========================================
//
// Das Kernkonzept "Code ist Daten" (Homoikonizität).
// Der Code `(plus 1 2)` ist *Daten* (ein Aufruf-Objekt).
// Der Code `[1 2 3]` ist *Daten* (ein Vektor-Objekt).
//
// Metaprogrammierung ist die Fähigkeit, Code zu schreiben,
// der *diese Datenstrukturen (Code) generiert*.

// --- 7.1 Primitives für Metaprogrammierung ---

// (quote expr)
// -> Stoppt die Ausführung von `expr` und gibt es als Daten zurück.
// -> (quote (plus 1 2))  -> (plus 1 2)  [als Daten, nicht 3!]
//
// Reader-Macro (Syntax-Zucker):
// '(plus 1 2)
// -> Ist identisch zu (quote (plus 1 2))
//
// (eval data)
// -> Nimmt Daten, die Code repräsentieren, und führt sie aus.
// -> (eval '(plus 1 2)) -> 3


// --- 7.2 Makros (Compile-Zeit-Code-Generierung) ---
//
// Ein Makro ist eine Funktion, die *während des Kompilierens*
// ausgeführt wird. Sie nimmt Code (Daten) entgegen und
// gibt *neuen Code* (Daten) zurück, der dann kompiliert wird.
//
// (macro <name> [param-vektor] (body ...))
//
// BEISPIEL: Wir definieren ein `(unless condition body)` Makro
//
// (macro unless [cond body]
//    // Wir generieren einen '(cond ...)' Ausdruck:
//    '(cond
//        (not cond) body    // Wenn (not cond) wahr ist, führe body aus
//        _          -> nil   // Sonst: tue nichts (gib nil zurück)
//    )
// )
//
// // Verwendung:
// (unless (eq 2 2)
//    ! (print "2 ist nicht 2") !  // Wird nicht ausgeführt
// )
//
// (unless (eq 1 2)
//    ! (print "1 ist nicht 2") !  // Wird ausgeführt
// )
//
// Der Compiler ersetzt den `(unless ...)`-Aufruf *vor* der Ausführung
// durch den `(cond ...)`-Block, den das Makro generiert hat.