
fn main() {
    let mut array: [i32; 5] = [5,2,3,4,1];

    print!("{:?} \n",array);
    print!("/////////// \n",);

    for mut i in 0..4 {
        loop {
            if array[i] > array[i+1] {
                let temp = array[i];
                array[i] = array[i+1];
                array[i+1] = temp;
            }else{
                break;
            }
            if i ==0{
                break;
            }
            i-=1;
        }
        
        i+=1; 
        if i ==4 {
            break;
        }
    }

    print!("{:?} \n",array);
    print!("/////////// \n",);


}

