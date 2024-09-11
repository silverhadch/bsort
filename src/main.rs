fn main() {
    let mut finish = false;
    let mut array_index: usize = 0; // usize sind für Arrays optimiert
    let mut switch: i32 = 0;
    let mut init = true;

    let mut arr: [i32; 15] = [11, 5, 8, 5, 2, 9, 5, 88, 6, 557, 5588, 96, 35, 66, 4];
    println!("Startarray: {:?}", arr);

    while !finish {
        let längearr = arr.len(); 

        if array_index < längearr - 1 && arr[array_index] > arr[array_index + 1] {
          
            arr.swap(array_index, array_index + 1);
            switch += 1;
        }
        
        array_index += 1; 
        if array_index >= längearr - 1 {
            if switch == 0 && !init { 
                finish = true; 
                println!("Ergebnis: {:?}", arr);
            }
            
            array_index = 0;
            switch = 0;
            init = false;
        }
    }
}
