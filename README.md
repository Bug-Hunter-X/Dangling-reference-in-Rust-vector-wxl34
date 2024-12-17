# Dangling Reference in Rust Vector

This repository demonstrates a common error in Rust involving dangling references to elements in a dynamically sized vector.  The code showcases how modifying a vector after borrowing one of its elements can lead to undefined behavior and potential crashes. The solution demonstrates safe ways to avoid this issue.

## Bug

The `bug.rs` file contains code that attempts to print the value of an element in a vector after the vector has been modified. This results in a dangling pointer because the memory location that the reference points to may be invalidated.  This is not caught by the compiler, and can lead to unpredictable behavior.

## Solution

The `bugSolution.rs` file provides corrected versions of the code, using safer methods like cloning or iterating to avoid the problem of dangling references.