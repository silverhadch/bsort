fn main() {
    let mut finish: bool = false;
    let mut array_index: usize = 0; // Verwende usize für Array-Indizes
    let mut j: i32 = 0;
    let mut init: bool = true;

    let mut arr: [i32; 15] = [11, 5, 8, 5, 2, 9, 5, 88, 6, 557, 5588, 96, 35, 66, 4];
    println!("Startarray: {:?}", arr);

    while !finish {
        let längearr: usize = arr.len(); // Verwende usize für Längen
        if array_index < längearr - 1 && arr[array_index] > arr[array_index + 1] {
            // Tausche die Werte, wenn das aktuelle Element größer ist als das nächste
            arr.swap(array_index, array_index + 1);
            j += 1; // Zähle die Anzahl der Swaps
        }
        array_index += 1; // Gehe zum nächsten Element

        if array_index >= längearr - 1 {
            if j == 0 && !init { // Wenn keine Swaps mehr gemacht wurden und es nicht die erste Iteration ist
                finish = true; // Sortierung ist abgeschlossen
                println!("Ergebnis: {:?}", arr);
            }
            // Setze die Schleifenvariablen für die nächste Iteration zurück
            array_index = 0;
            j = 0;
            init = false;
        }
    }
}

