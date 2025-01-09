fn Fibonacci(n:usize)->[usize;10]{
    let mut fib=[0;10];
    fib[0]=0;
    if(n==0) {
        return fib;
    }
    fib[1]=1;
    for i in 2..n{
        fib[i]=fib[i-2]+fib[i-1];
    }
     fib
}
fn main() {
    let mut n=10;
    let mut fib=Fibonacci(n);
    println!("{:?}",fib);
}
