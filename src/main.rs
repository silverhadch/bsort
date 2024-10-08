fn main() {
    let mut finish = false;
    let mut array_index: usize = 0;
    let mut switch: i32 = -1;

    let mut arr: [i32; 15] = [11, 5, 8, 5, 2, 9, 5, 88, 6, 557, 5588, 96, 35, 66, 4];
    println!("Start array: {:?}", &arr);

    while !finish {
        let length = arr.len();

        switch = 0; // Reset switch count for each pass

        for i in 0..length - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                switch += 1;
            }
        }

        if switch == 0 {
            finish = true;
            println!("Sorted result: {:?}", &arr);
        }
    }
}
