pragma circom 2.1.5;

template Example () {
    signal input a;
    signal input b;
    signal output c;
    
    c <== a + b ;
    c === 77;
}

component main = Example();