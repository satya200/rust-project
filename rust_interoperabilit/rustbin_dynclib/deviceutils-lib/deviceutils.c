#include "deviceutils.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_SIZE 256

DeviceProp get_device_prop_struct(const char* filepath, const char* key) {
    static char value[LINE_SIZE];
    static DeviceProp prop;
    prop.status = STATUS_ERR;
    prop.key = key;
    prop.value = NULL;

    FILE* file = fopen(filepath, "r");
    if (!file) return prop;

    char line[LINE_SIZE];
    size_t key_len = strlen(key);

    while (fgets(line, sizeof(line), file)) {
        if (strncmp(line, key, key_len) == 0 && line[key_len] == '=') {
            char* val = strchr(line, '=');
            if (val) {
                val++;
                val[strcspn(val, "\n")] = '\0';
                strncpy(value, val, sizeof(value));
                prop.value = value;
                prop.status = STATUS_OK;
                break;
            }
        }
    }

    fclose(file);
    return prop;
}

