# With JEP
This is pretty straight forward, include JEP as a dependency and most of the interop work is done. Build the project with the python libraryalready present in the PYTHONPATH and everything is good to go.

# With RUST based custom bridge
With JEP we get the bridge already built in C, here we are building the bridge and in addition to that we are giving nice wrappers around the methods which java programs have to use. We can avoid dirty looking definition that we get with JNI instead we can use https://github.com/giovanniberti/robusta. This library was used to understand how to return `List<String>` from `Vec<String>`

## Follow the following steps to run the project:
- install the python project with(this step is common for both approaches): `pip install .` 
- build rust project with: `cargo build --release`
- build java project with: `mvn clean package`
- run java program with: `LD_LIBRARY_PATH="/home/gautam/dc/python-interop/with_rust/python_interop/target/release:$LD_LIBRARY_PATH" java -cp python-interop-1.0-SNAPSHOT.jar com.highradius.interop.App`. Change the library path to whereever your shared library is built

