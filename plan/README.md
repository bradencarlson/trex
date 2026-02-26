# Multiple Passes

## Proposed Solution

I believe that the following procedure could successfully detect 
when a second pass is needed: 

1. If `jobname.aux` exists: compute it's checksum. 
2. Compile
3. Check `jobname.aux`, 
    1. If it did not exist before, search it 
    for `\newlabel` commands, if these exist, recompile, then compute 
    it's checksum. 
    2. If it did exist before, compute it's checksum, if it
    differs, recompile.

A better description is found in `pass-logic.png` where I have created a 
diagram which specifies exactly when TreX will need to compile a document. 
Currently, `bibtex` is not supported. 
