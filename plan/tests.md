# Tests

The following scenarios need to be accounted for in whatever procedure
that is created for compiling the TeX documents: 

- Regular TeX file (no references, citations, etc.)
- TeX file with references 
    - references change (two passes) ... ok
    - references do not change (one pass) ... ok
    - from clean ... ok
- TeX file with references/bibliography (needs bibtex, plus several passes)
    - references have changed ... ok (runs bibtex more than needed)
    - citations have changed ... ok
    - both change ... ok
    - from clean ... ok (runs bibtex more than needed)
