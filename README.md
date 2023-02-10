# Pedago Project : Simplex visualisation

Authors :

- Aloïs Rautureau
- Elie Dumont
- Paul Adam

---

## **Introduction**
This is a 1-week project to visualize the simplex algorithm done during a computer science project. 
    
We chose the language Rust for its efficiency and its security. It is also pretty simple to then compile our GUI to Webassembly to run it in a browser.

## **How to run**
> ### Webpage
The webpage is available at https://aloisrautureau.github.io/simplex/. Everything is explained on the webpage to use the tool. It is Rust compiled to WebAssembly and runs in a browser very smoothly.

> ### To run it yourself
If you want to compile yourself this Rust project, you need to install Rust and Cargo. You can then run the following command to compile the project :

```
>> cargo run
```

About testing, we have tested the major part of the code. To run the tests, you can run the following command :

```
>> cargo test
```

## What has been done
On the website, you can enter a linear problem (with a text) and the programm will solve it. According to the dimension of the problem the output will be different :
- 2D : The output will be a 2D graph with the points of the simplex and the solution.
- 3D : The output will be a 3D graph with the points of the simplex and the solution.
- >=4D : The output will be a terminal-print like. Step by step you can see the point where you are and the next point where you go.

---

## Structure of the project

We have done this project entirely in Rust. The project is divided into two main parts :

In ```/algui``` you can find the very simple graphical lib to display the points in 2D and 3D.

- In ```/src```, you can find the implementation of the simplex algorithm that calls the ```algui``` lib to show the simplex as well as the current point.
- In ```/src/contraints.rs``` : Implementation of the constraints, the struct used to describe inequalities.
- In ```/src/linear.rs``` : Implementation of linear_function, the struct used to describe the linear function to be maximised and the linear function in the constraints as each side of a constraint is implemented as a linear function.
- ```/src/lib.rs``` : Implementation of the simplex structure and the skull of the algorithm.

---

## TODO

- Finish Algui
- Test completely the simplex algorithm


- Gestion of the error (error type)
- transform linear_function to a matrix
- compute a point of the simplex with the contraints
- test everithing
