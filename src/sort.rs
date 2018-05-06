/* Quick sort 
 * Only for i64 type element yet
 * Todo: Generic implementation
 */

fn partition(arr: &mut [i64], low: usize, high: usize){
    let pivot:i64 = arr[high];
    let i: usize = low - 1;
    let j: usize;

    for j in low..high {
        if arr[j] <= pivot {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i+1 as usize, high as usize);

    i+1
}

fn quickSort(arr: &mut [i64], low: usize, high: usize){
    if low < high {
        let mut mid = partition(arr, low, high);
        quickSort(arr, low, mid-1);
        quickSort(arr, mid+1, high);
    }
}


pub fn qSort(arr: &mut [i64]){
    let high: usize = arr.size()-1;
    quickSort(arr, 0 as usize, high as usize);
}