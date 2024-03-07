fn main() {
    // let variable = moyenne(10.0,0.0);
    // println!("{}",variable);

    // let variable = moyenne(10.0,0.0);
    // println!("{}",variable);

    // let my_rectangle : Rectangle =Rectangle { lenght: (10.0), width: (10.0) };

    // let my_rectangle2 = my_rectangle.clone();
    // let my_rectangle3 = my_rectangle.clone();
    
    // let my_perimetre = perimetre(my_rectangle2);
    // println!("{}", my_perimetre);
    
    // let perimetre = perimetre(my_rectangle3);
    // println!("{}", perimetre);
    //print_reference();

    let mut var1 = 10.0;
    let mut var2 = 5.0;

    println!("{} {}", var1,var2);
    swap(&mut var1,&mut var2);
    println!("{} {}", var1,var2);
    swap(&mut var1,&mut var2);
    println!("{} {}", var1,var2);
}


fn moyenne(x : f64, y : f64) -> f64{
    let la_moyenne : f64 = (x+y)/2.0;
    la_moyenne
}

#[derive(Clone)]
struct Rectangle {
    lenght : f64,
    width : f64,
}

fn perimetre(rectangle :Rectangle) -> f64 {
    rectangle.lenght*rectangle.width
}

fn perimetre2(rectangle :&Rectangle) -> f64 {
    rectangle.lenght*rectangle.width
}

fn print_reference(){
    let x : i32 = 18;
    // let ref1: &i32 = &x;
    // let ref2: &i32 = &x;
    let mut ref3: &i32 = &x;
    let mut ref4: &i32 = &x;

    println!("{}, {}", ref3, ref4);
}

fn swap(float1 :&mut f32, float2 :&mut f32){
    let temp = *float1;
    *float1 = *float2;
    *float2 = temp;
}