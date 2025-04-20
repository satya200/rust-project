#include <stdio.h>
#include <stdlib.h>

// Declaration of the Rust function
extern char* get_device_prop(const char* path, const char* key);

int main() {
    const char* file_path = "/mnt/rust-dev/rust_dynlib_ctime/device.prop";
    const char* key = "device_name";

    char* result = get_device_prop(file_path, key);

    if (result) {
        printf("%s = %s\n", key, result);
    } else {
        printf("Property not found.\n");
    }

    return 0;
}

