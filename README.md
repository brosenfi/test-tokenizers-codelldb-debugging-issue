# Tokenizers CodeLLDB Debug Issue

## Detail
Reproducing the current seg-fault problem occurring while trying to debug the Tokenizers crate with the CodeLLDB extension in VSCode.  A valid tokenizer.json is supplied in the workspace folder for the test - this was obtained from the bge-large-en-v1.5 model .

## Reproducing
Open the workspace file in the repository root using VSCode - if you have the dev containers extension, it should prompt to re-open the workspace in the dev-container (do this for ease of use).  Use the "Test Debug Issue" launch command and note that the debugger crashes with a SIGSEGV after running line 15 in main.rs .

## Direct Debugging in LLDB
In the dev-container run the following commands in a terminal in the workspace folder to start lldb with CodeLLDB's copy:
```bash
cargo build
/home/vscode/.vscode-server/extensions/vadimcn.vscode-lldb-1.11.6/lldb/bin/lldb ./target/debug/test-issue
```
Then issue the following commands in the terminal debug session:
```lldb
# log enable lldb all # Enable this if you want all the debug output
breakpoint set --file main.rs --line 15
run --tokenizer-json-path tokenizer.json
thread step-over
thread step-over
thread continue
quit
```
Note debugging is successful using LLDB directly on the terminal - no error / seg-fault.

## Executing to Completion Outside of the Debugger
Run the following command to show the code run to completion without error when the debugger is not attached (in the workspace folder):
```bash
./target/debug/test-issue --tokenizer-json-path tokenizer.json
```
