// A simple program to print the first 10 Fibonacci humbers

let a = 0;
let b = 1;

for _ in range(10) {
    println(a);

    let c = a;
    a = b;
    b += c;
}
