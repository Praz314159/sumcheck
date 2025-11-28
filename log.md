# Project Notes 

## TO DO 

1. 

## LOG 

1. The goal right now is to try and implement evaluation of the multilinear extension from scratch. I'm pretty optimistic that I can do this without notes. This would be a good validation of my understanding and skills. But we'll see! It's important to have profiling setup. I'm going to give this fist session 3 hours worth of time and see how it goes. 

Recall the set up for a multilinear polynomial: 
    - We have a map f: B^w --> F from the w-dimensional boolean hypercube to a finite field.  
    - Then the goal, given the map of f and a point z in F, to compute the evalution of the MLE of f at z 

Ok, this doesn't seem too bad. Actually, we can make a multilinear extension as a trait. The trait should have a function for implementing 
