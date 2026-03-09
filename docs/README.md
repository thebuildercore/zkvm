

# 1 trial 
created a fake IR and then ran the rust command to convert it from branching into single polynomial the output.tsx showed it

# 2 Trial 
trying to get the real .ll file from target.rs
by running this :   rustc --emit=llvm-ir --crate-type=lib target.rs 
and got target.ll (  br i1 %_3 ) this is the exact shit that slow down zkvms
