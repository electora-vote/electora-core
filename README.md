Building
--------

Install the task runner, [just](https://github.com/casey/just).

Run `just` from the command line.

This will compile the rust lib to three wasm targets in the `pkg` folder.

Testing
-------

Start a web server in the root folder of the project. e.g.

```
python -m http.server
```

point your browser at http://localhost and you should see and alert box pop up with
the message "Hello, WebAssembly!"
