# Python wrapper for Indy Besu VDR

This is thin python package created on top of Indy Besu bindings generated using [Uni-FFI](../../uniffi/README.md).

### Requirements

* Python of version 3.6.3 or higher.

### Build package

1. Bindings building:
    * Build bindings as describe in uniffi [README.md](../../uniffi/README.md).
    * Copy `uniffi/out/indy_besu_vdr` file and put into `wrappers/python/indy_besu_vdr` folder.
    * Copy `uniffi/target/release/libindy_besu_vdr_uniffi.dylib` file and put into `wrappers/python/indy_besu_vdr` folder.
2. Package building:
    * Run the following commands:
       ```
        python3 -m pip install --upgrade build
        python3 -m build
        ```

### Run demo

You can find and run sample script [here](./demo/test.py)

* Update constant defined at hte top of the file
* Run sample using the following command:
   ```
   pip3 install eth_keys
   python3 -m demo.test
   ```