# Pedago Project : Simplex visualisation

Authors :

- Aloïs Rautureau
- Elie Dumont
- Paul Adam

---

## **Introduction**
This is a 1-week project to visualize the simplex algorithm done during a computer science project. 
    
We have choose the language Rust for its efficiency and its security. Also it is pretty simple to then compile our GUI to Webassembly to run it in a browser.

## **How to run**
> ### Webpage
The webpage is available at https://aloisrautureau.github.io/simplex/. Everything is explained on the webpage to use the tool. It is Rust compiled to WebAssembly and run in a browser very smoothly.

> ### To run it your self
If you want to compile yourself this Rust project, you need to install Rust and Cargo. Then you can run the following command to compile the project :

```
>> cargo run
```

For testing, we have tested the major part of the code. To run the tests, you can run the following command :

```
>> cargo test
```

## What has been done
On the website, you can enter a linear problem (with a text) and the programm will solve it. According to the dimension of the problem the output will be different :
- 2D : The output will be a 2D graph with the points of the simplex and the solution.
- 3D : The output will be a 3D graph with the points of the simplex and the solution.
- >=4D : The output will a terminal-print like. Step by step you can see the point where you where and the next point where you go.

---

## Structure of the project

We have completely done this project in Rust. The project is divided in two main parts :

In ```/algui``` you can found the very simple graphical lib to display the points in 2D and 3D.

- In ```/src```, you can found the implementation of the simplex algorithm that call the ```algui``` lib to show the simplex, and the current point.
- In ```/src/contraints.rs``` : Implementation of the contraints, the struct use to describe inequalities.
- In ```/src/linear.rs``` : Implementation of linear_function the struct use to describe the linear funtction to maximise aswell as the linear function in the contraints.
- ```/src/lib.rs``` : Implementation of the simplex structure and of everithing.

---

## TODO

- Finish Algui
- Test completely the simplex algorithm


- Gestion of the error (error type)
- transform linear_function to a matrix
- compute a point of the simplex with the contraints
- test everithing
