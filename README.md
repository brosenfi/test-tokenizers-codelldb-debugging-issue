# Rust Tokenizers Module LLDB Debug Issue

## Detail
Reproducing the current seg-fault problem occurring while trying to debug the Tokenizers crate with the CodeLLDB extension in VSCode.  A valid tokenizer.json is supplied in the workspace folder for the test - this was obtained from the bge-large-en-v1.5 model .

## Reproducing
Open the workspace file in the repository root using VSCode - if you have the dev containers extension, it should prompt to re-open the workspace in the dev-container (this will build and install the latest LLDB release at this time = 21.1.3 - this will take a while).  Use the "Test Debug Issue" launch command and note that the debugger crashes with a SIGSEGV after running line 15 in main.rs .

## Reproducing Directly in LLDB (in a Terminal)
In the dev-container run the following commands in a terminal in the workspace folder to start lldb with CodeLLDB's copy:
```bash
cargo build
lldb ./target/debug/test-issue
```
Then issue the following commands in the terminal debug session:
```lldb
# log enable lldb all # Enable this if you want all the debug output
breakpoint set --file main.rs --line 15
run --tokenizer-json-path tokenizer.json
frame variable
thread step-over
thread step-over
frame variable
```
Note LLDB crashes when inspecting the current frame's state.

## Executing to Completion Outside of the Debugger
Run the following command to show the code run to completion without error when the debugger is not attached (in the workspace folder):
```bash
./target/debug/test-issue --tokenizer-json-path tokenizer.json
```
