# GPIO Remote Protocol

The API consists of 4 functions, listed here:

```txt
open_input(id: u16) -> in_handle
open_output(id: u16) -> out_handle
input_read(handle: in_handle) -> gpio_value
output_set(handle: out_handle, val: gpio_value)
```

for this, we need the receiver to keep a map of all the local, real handlers
sounds easy (?????)
