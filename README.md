Building and Testing
--------------------

Install the task runner, [just](https://github.com/casey/just).

Run `just` from the command line.

This will:

* compile the rust lib to three wasm targets in the `pkg` folder
* build a python wheel in the `target` folder which wraps the rust lib
* run the test suite

You can also test the web build:

Start a web server in the root folder of the project. e.g.

```
python -m http.server
```

point your browser at http://localhost and you should see and alert box pop up with
the message "Hello, WebAssembly!"
