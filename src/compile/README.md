# Compile Module

All the actual commands for compiling a document are run here. The `mod.rs`
file should not contain any of that logic of generating the code and running
it, but rather that should belong to the submodules of this one. 

### Writing Submodules

When adding a new document type, all the logic for generating and compiling
that document type should be located in a submodule. This submodule must
define at least the following three public methods (with the following
signatures):
- `run(cmd: &CMD)` this method must perform (or call private functions which
  perform) all neccessary steps to compile the document and provide any
  errors for the user to see. 
- `clean()` this method must clean up all auxiliary files generated in the
  compilation process (if any).
- `get_code(...)` this method can take arguments, such as the outline, range of
  files to be included (i.e. [2,3,4]), etc. and must return the ENTIRE code
  which could be used to compile the document without TreX.
There may be additional public methods if needed for writing tests in the
`mod.rs` file, but only the above three are required. 
